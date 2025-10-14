#!/usr/bin/env bash

if [ ${BASH_SOURCE[0]} == "${0}" ]; then
    echo "This script is meant to be sourced, not executed."
    return 1
fi

FEATURE_INSTALL_DIR="/opt/devcontainer-features/cargo-tools"
