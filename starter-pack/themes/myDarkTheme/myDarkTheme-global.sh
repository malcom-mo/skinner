# Terminal.app
osascript -e "tell application \"Terminal\"
    set default settings to settings set \"Red Sands\"
    set startup settings to settings set \"Red Sands\"
    set current settings of tabs of windows to settings set \"Red Sands\"
    end tell"

# Neovim
for s in $(nvr --serverlist)
do
    nvr -s --nostart --servername $s \
        --remote-send '<esc>:set background=dark<enter>:colorscheme retrobox<enter>' &
done

# tmux
if pgrep -xq "tmux"
then
    tmux source-file ~/.config/skinner/themes/myDarkTheme/dark-tmux.conf
fi
