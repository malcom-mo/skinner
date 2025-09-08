# Skinner
Skinner is a small tool to manage the colors of your terminal applications.

🎨 Works with Terminal.app, tmux, vim, bat, ...

🎨 `skinner activate myTheme` does three things:
- deactivates your previous theme.
- runs your provided `myTheme-global.sh`, allowing you to send remote commands like `tmux source myTheme.conf`.
- runs your provided `myTheme-per-shell.sh` in each open zsh/bash/fish shell, allowing you to set environment variables like `BAT_THEME=myTheme` or shell aliases.

🎨 Skinner automatically activates your themes when your system switches between light and dark mode, building on [dark-mode-notify](https://github.com/bouk/dark-mode-notify).

## Installation

    brew install malcom-mo/tap/skinner

Confirm it‘s installed:

    skinner --help

Add the following to your shell configuration files and run it in your currently open sessions:

```bash
# zsh or bash (.zshrc, .bashrc, .bash_profile)
source <(skinner setup)

# fish (fish.config)
skinner setup --fish | source
```

Start the background service for dark mode sync:

    brew services start skinner

## Configuration
Place all your `myTheme-global.sh` and `myTheme-per-shell.sh` scripts in `~/.config/skinner/themes/myTheme`, eg, `~/.config/skinner/themes/myTheme/myTheme-global.sh`.

> [!NOTE]
> Deactivating themes just means activating a special theme called `off`.
> You **optionally specify the deactivation commands** the same way as for other themes: `off-global.sh` and `off-per-shell.sh` in `~/.config/skinner/themes/off/`.

> [!TIP]
> See `starter-pack/` in this repository for example commands to put in these scripts.

You can set the following options in `~/.config/skinner/skinner.conf`:

```yaml
themes: ~/.config/skinner/themes

# The signal skinner gives to running shells to load -per-shell.sh scripts.
# Reasonable options are USR1, USR2, URG, WINCH, IO, VTALARM, PROF.
# Note that unprepared shells might die from USR1/2.
# URG etc. are safe and usually not used by shells otherwise.
signal: URG

# Themes that are activated when macOS goes to light or dark mode
# or when you say `skinner activate light` or `skinner activate dark`
light: myLightTheme
dark: myDarkTheme
```

## Related projects
- [dark-mode-notify](https://github.com/bouk/dark-mode-notify) -- run commands when macOS changes to dark mode.
  *This project includes code derived from dark-mode-notify, copyright (c) Bouke van der Bijl, licensed under the [MIT License](https://github.com/bouk/dark-mode-notify/blob/main/LICENSE).*
- [Nightfall](https://github.com/r-thomson/Nightfall/) -- activate macOS dark mode from the menu bar.
