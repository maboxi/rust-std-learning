#!/usr/bin/env bash
set -e

FEATURE_INSTALL_DIR="/opt/devcontainer-features/cargo-tools"
. "$FEATURE_INSTALL_DIR/scripts/feature_options.sh"

echo -n "\
----------------------------------------------------------------
-
-   Post create script for cargo-tools feature
-
-   Options:
-     Cargo tools to install: 
-       ${TOOLS_LIST}
-
----------------------------------------------------------------

"

. "$FEATURE_INSTALL_DIR/scripts/install_tools.sh"
