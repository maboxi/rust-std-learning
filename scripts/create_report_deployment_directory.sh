#!/usr/bin/env bash

set -e

if [ $# -lt 1 ]; then
  echo "Usage: $0 <ref name>"
  exit 1
fi

REF_NAME="$1"
COVERAGE_REPORT_PATH="${REPORT_PATH:-"target/tarpaulin/tarpaulin-report.html"}"
COVERAGE_REPORT_DEPLOYMENT_PATH="${COVERAGE_REPORT_DEPLOYMENT_PATH:-"target/tarpaulin/pages-deployment/coverage-report/$REF_NAME"}"

if [ ! -f "$COVERAGE_REPORT_PATH" ]; then
    echo "Coverage report not found at $COVERAGE_REPORT_PATH"
    exit 2
fi

echo "Creating deployment directory for ref '$REF_NAME' at '$COVERAGE_REPORT_DEPLOYMENT_PATH'"

mkdir -p "$COVERAGE_REPORT_DEPLOYMENT_PATH"
cp target/tarpaulin/tarpaulin-report.html "$COVERAGE_REPORT_DEPLOYMENT_PATH/index.html"