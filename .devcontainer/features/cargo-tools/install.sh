#!/usr/bin/env bash
set -e

SCRIPT_DIR="$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

# Devcontainer configuration
USER=${_REMOTE_USER:-"$(whoami)"}

# Feature options
TOOLS=${TOOLS:-""}
TOOLS_LIST=("${TOOLS//,/ }")

echo "Effective user: $(whoami)"
echo "Remote user: $USER"

if ! which -s cargo; then
    echo "Cargo not found!"
    exit 1
fi

. "$SCRIPT_DIR/scripts/config.sh"

if [ -d "$FEATURE_INSTALL_DIR" ]; then
    rm -r "$FEATURE_INSTALL_DIR"
fi
mkdir -p "$FEATURE_INSTALL_DIR"
echo "Copying temporary feature directory $SCRIPT_DIR into installation directory $FEATURE_INSTALL_DIR"
cp -r -T "$SCRIPT_DIR" "$FEATURE_INSTALL_DIR"

sed -e "s/^TOOLS_LIST=.*$/TOOLS_LIST=\"${TOOLS_LIST}\"/" "$FEATURE_INSTALL_DIR/scripts/feature_options_template.sh" > "$FEATURE_INSTALL_DIR/scripts/feature_options.sh"

chown -R $USER:$USER "$FEATURE_INSTALL_DIR"
