use clap::{Parser, Subcommand};
use env_logger::Builder;
use log::{LevelFilter, debug, error, info};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, exit};

#[derive(Parser)]
#[command(name = "skinner")]
#[command(about = "Terminal theme manager with macOS dark mode sync")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
}

#[derive(Subcommand)]
enum Commands {
    /// List available themes
    #[command(visible_aliases = ["ls"])]
    List,
    /// Activate a theme
    #[command(visible_aliases = ["a"])]
    Activate {
        /// Theme name to activate
        theme: String,
    },
    /// Deactivate current theme (same as 'activate off')
    #[command(visible_aliases = ["d", "off"])]
    Deactivate,
    /// Generate shell setup code (--fish for fish syntax instead of bash/zsh)
    Setup {
        /// Generate fish syntax instead of bash/zsh
        #[arg(long)]
        fish: bool,
    },
}

#[derive(Deserialize, Serialize, Debug, Default)]
struct Config {
    #[serde(default = "default_theme_dir")]
    themes: PathBuf,
    #[serde(default = "default_signal")]
    signal: String,
    light: Option<String>,
    dark: Option<String>,
}

fn default_theme_dir() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(format!("{}/.config/skinner/themes", home))
}

fn default_signal() -> String {
    "URG".to_string()
}

struct SkinnerApp {
    config_file: PathBuf,
    current_theme_file: PathBuf,
}

impl SkinnerApp {
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let home = std::env::var("HOME").map_err(|_| "HOME environment variable not set")?;

        let config_dir = PathBuf::from(format!("{}/.config/skinner", home));
        let config_file = config_dir.join("skinner.conf");
        let current_theme_file = config_dir.join("current_theme");

        Ok(Self {
            config_file,
            current_theme_file,
        })
    }

    fn load_config(&self) -> Result<Config, Box<dyn std::error::Error>> {
        info!("Using config file: {}", self.config_file.to_string_lossy());
        if !self.config_file.exists() {
            info!("Config file not found, using defaults");
            return Ok(Config::default());
        }

        let content = fs::read_to_string(&self.config_file)?;
        let config: Config =
            serde_yaml::from_str(&content).map_err(|e| format!("Invalid config file: {}", e))?;

        Ok(config)
    }

    fn get_current_theme(&self) -> Result<String, Box<dyn std::error::Error>> {
        if !self.current_theme_file.exists() {
            return Ok("off".to_string());
        }

        let theme = fs::read_to_string(&self.current_theme_file)?
            .trim()
            .to_string();

        Ok(theme)
    }

    fn set_current_theme(&self, theme: &str) -> Result<(), Box<dyn std::error::Error>> {
        fs::write(&self.current_theme_file, theme)?;
        Ok(())
    }

    fn execute_global_script(&self, theme_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let script_name = format!(
            "{}-global.sh",
            theme_dir
                .file_name()
                .ok_or("Invalid theme directory")?
                .to_str()
                .ok_or("Invalid theme directory string")?
        );

        let script_path = theme_dir.join(&script_name);

        if !script_path.exists() {
            info!("Global script not found: {}", script_path.display());
            return Ok(());
        }

        let output = Command::new("bash").arg(&script_path).output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            error!("Script execution failed: {}", stderr);
            return Ok(());
        }

        Ok(())
    }

    fn send_signals(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config = self.load_config()?;
        // login shells are often prefixed with -
        let shell_names = ["zsh", "bash", "fish", "-zsh", "-bash", "-fish"];
        for name in shell_names {
            let output = match Command::new("killall")
                .arg(format!("-{}", config.signal))
                .arg("--")
                .arg(format!("{}", name))
                .output()
            {
                Ok(output) => output,
                Err(e) => {
                    error!(
                        "Could not run killall -{} -- {}: {}",
                        config.signal, name, e
                    );
                    continue;
                }
            };

            if output.status.success() {
                info!("Sent {} to all {} processes", config.signal, name);
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                // It is expected that some commands return "not found" when no shell if active
                debug!(
                    "Error from sending {} to {}: {}",
                    config.signal, name, stderr
                );
            }
        }

        Ok(())
    }

    fn list_themes(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let config = self.load_config()?;

        if !config.themes.exists() {
            info!(
                "Themes directory {} not found",
                config.themes.to_string_lossy()
            );
            return Ok(Vec::new());
        }

        let mut themes = Vec::new();

        for entry in fs::read_dir(&config.themes)? {
            let entry = entry?;
            let path = entry.path();

            if !path.is_dir() {
                continue;
            }

            let theme_name = match path.file_name().and_then(|n| n.to_str()) {
                Some(name) => name,
                None => continue,
            };

            let has_global = path.join(format!("{}-global.sh", theme_name)).exists();
            let has_per_shell = path.join(format!("{}-per-shell.sh", theme_name)).exists();

            if has_per_shell || has_global {
                themes.push(theme_name.to_string());
            }
        }

        themes.sort();
        Ok(themes)
    }

    fn setup_bash(&self) -> Result<String, Box<dyn std::error::Error>> {
        let config = self.load_config()?;
        Ok(format!(
            r#"# --- Added by skinner ---
_skinner_activate_theme() {{
  if [[ -f "$HOME/.config/skinner/current_theme" ]]; then
    theme=$(cat ~/.config/skinner/current_theme)
    if [[ -f "$HOME/.config/skinner/themes/$theme/$theme-per-shell.sh" ]]; then
      source "$HOME/.config/skinner/themes/$theme/$theme-per-shell.sh"
    fi
  fi
}}
_skinner_activate_theme
trap _skinner_activate_theme {}
# --- End skinner ---"#,
            config.signal
        ))
    }

    fn setup_fish(&self) -> Result<String, Box<dyn std::error::Error>> {
        let config = self.load_config()?;
        Ok(format!(
            r#"# --- Added by skinner ---
function _skinner_activate_theme --on-signal {}
  if test -f "$HOME/.config/skinner/current_theme"
    set theme (cat ~/.config/skinner/current_theme)
    if -f "$HOME/.config/skinner/themes/$theme/$theme-per-shell.sh"
      source "$HOME/.config/skinner/themes/$theme/$theme-per-shell.sh"
    end
  end
end
_skinner_activate_theme
# --- End skinner ---"#,
            config.signal
        ))
    }

    fn run(&self, command: Commands) -> Result<(), Box<dyn std::error::Error>> {
        match command {
            Commands::Setup { fish } => {
                if fish {
                    println!("{}", self.setup_fish()?);
                } else {
                    // zsh or bash
                    println!("{}", self.setup_bash()?);
                }
            }

            Commands::Deactivate => {
                self.run(Commands::Activate {
                    theme: "off".to_string(),
                })?;
            }

            Commands::Activate { theme } => {
                let config = self.load_config()?;

                // If not "off", first deactivate current theme
                if theme != "off" {
                    let current = self.get_current_theme()?;
                    if current != "off" {
                        self.run(Commands::Activate {
                            theme: "off".to_string(),
                        })?;
                    }
                }

                let actual_theme = match theme.as_str() {
                    "light" => config
                        .light
                        .ok_or("No light theme configured in skinner.conf")?,
                    "dark" => config
                        .dark
                        .ok_or("No dark theme configured in skinner.conf")?,
                    _ => theme,
                };
                let theme_dir = config.themes.join(&actual_theme);
                self.execute_global_script(&theme_dir)?;

                self.set_current_theme(&actual_theme)?;
                self.send_signals()?;

                if actual_theme == "off" {
                    info!("Theme deactivated");
                } else {
                    info!("Activated theme: {}", actual_theme);
                }
            }

            Commands::List => {
                let themes = self.list_themes()?;
                let current = self.get_current_theme()?;

                if themes.is_empty() {
                    info!("No themes found");
                    return Ok(());
                }

                for theme in themes {
                    if theme == current {
                        println!("{} (*)", theme);
                    } else {
                        println!("{}", theme);
                    }
                }
            }
        }

        Ok(())
    }
}

fn main() {
    let cli = Cli::parse();

    let log_level = match cli.verbose {
        0 => LevelFilter::Error,
        1 => LevelFilter::Info,
        _ => LevelFilter::Debug,
    };

    Builder::from_default_env().filter_level(log_level).init();

    let app = match SkinnerApp::new() {
        Ok(app) => app,
        Err(e) => {
            error!("Error: {}", e);
            exit(1);
        }
    };

    if let Err(e) = app.run(cli.command) {
        error!("Error: {}", e);
        exit(1);
    }
}
