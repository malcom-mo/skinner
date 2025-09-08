# vim
# Our vim config can use this to load the right theme on startup
export VIM_THEME=default

# tmux
# Our tmux config can use this to load the right theme on startup
export TMUX_THEME=~/.config/skinner/themes/myLightTheme/light-tmux.conf

# bat
export BAT_THEME=Coldark-Cold
# alias bat="bat --theme Coldark-Cold"

# bat as man pager
export MANPAGER="sh -c 'col -bx | bat --language man --style plain --theme Coldark-Cold'"

# bat as git pager
export GIT_CONFIG_COUNT=1
export GIT_CONFIG_KEY_0="core.pager"
export GIT_CONFIG_VALUE_0="bat --style=plain --theme=Coldark-Cold"

# fzf
export FZF_DEFAULT_OPTS="--color light"
