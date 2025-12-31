#!/bin/bash

# Gets home dir
if [ -n "$SUDO_USER" ]; then
    # Get the home directory of the original user
    # This works on most Linux/BSD systems
    ORIGINAL_HOME=$(getent passwd "$SUDO_USER" | cut -d: -f6)
else
    # Not running with sudo, use the current $HOME
    ORIGINAL_HOME="$HOME"
fi

INSTALL_LOC=/usr/bin/rsd-tui
RESOURCE_DIR="$ORIGINAL_HOME/.local/share/rsd-tui"

CLEAN=0
FORCE=0

while getopts "cf" opt; do 
    case $opt in
        c)
            CLEAN=1
            ;;
        f)
            FORCE=1
            ;;
        \?) # Handle invalid options
            echo "Invalid option: -$OPTARG" >&2
            exit 1
            ;;
    esac
done

if [ -e "$INSTALL_LOC" ]; then
    if [ $FORCE -eq 1 ]; then
        rm -f "$INSTALL_LOC"
    else
        echo "Desired Install Location: $INSTALL_LOC is already in use, use -f to forcefully install"
        exit 1
    fi
fi

if [ -e "$RESOURCE_DIR" ]; then
    if [ $FORCE -eq 1 ]; then
        rm -rf "$RESOURCE_DIR"
    else 
        echo "Desired Resource Location: $RESOURCE_DIR is already in use, use -f to forcefully install"
        exit 1
    fi
fi

# TODO: Change this to a curl from the build location when released
cp rsd-tui "$INSTALL_LOC"
mkdir "$RESOURCE_DIR"
touch "$RESOURCE_DIR/psd.bin"