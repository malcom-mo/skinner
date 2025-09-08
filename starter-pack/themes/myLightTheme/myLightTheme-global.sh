# Terminal.app
osascript -e "tell application \"Terminal\"
    set default settings to settings set \"Basic\"
    set startup settings to settings set \"Basic\"
    set current settings of tabs of windows to settings set \"Basic\"
    end tell"

# Neovim
for s in $(nvr --serverlist)
do
    nvr -s --nostart --servername $s \
        --remote-send '<esc>:set background=light<enter>:colorscheme default<enter>' &
done

# tmux
if pgrep -xq "tmux"
then
    tmux source-file ~/.config/skinner/themes/myLightTheme/light-tmux.conf
fi
