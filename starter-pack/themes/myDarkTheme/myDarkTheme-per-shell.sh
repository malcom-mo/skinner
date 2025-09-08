# vim
# Our vim config can use this to load the right theme on startup
export VIM_THEME=retrobox

# tmux
# Our tmux config can use this to load the right theme on startup
export TMUX_THEME=~/.config/skinner/themes/myDarkTheme/dark-tmux.conf

# bat
export BAT_THEME=Coldark-Dark
# alias bat="bat --theme Coldark-Dark"

# bat as man pager
export MANPAGER="sh -c 'col -bx | bat --language man --style plain --theme Coldark-Dark'"

# bat as git pager
export GIT_CONFIG_COUNT=1
export GIT_CONFIG_KEY_0="core.pager"
export GIT_CONFIG_VALUE_0="bat --style=plain --theme=Coldark-Dark"

# fzf
export FZF_DEFAULT_OPTS="--color dark"
