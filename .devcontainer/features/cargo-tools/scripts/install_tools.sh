#!/usr/bin/env bash
set -e

USER=${_REMOTE_USER:-"$(whoami)"}
TOOLS_LIST=${TOOLS_LIST:-""}

if which -s cargo; then
    if [ "$TOOLS_LIST" == "" ]; then
        echo "No cargo tools specified for installation."
        exit 0
    fi
    echo "Installing cargo tools: ${TOOLS_LIST}"
    cargo install $TOOLS_LIST
    echo "Cargo tools installed."
else
    echo "Cargo not found!"
    exit 1
fi