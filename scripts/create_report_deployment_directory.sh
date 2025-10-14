#!/usr/bin/env bash

set -e

if [ $# -lt 1 ]; then
  echo "Usage: $0 <ref name>"
  exit 1
fi

REF_NAME="$1"
COVERAGE_REPORT_ROOT="${COVERAGE_REPORT_ROOT:-"target/tarpaulin/"}"
COVERAGE_REPORT_FILE_PATH="${REPORT_PATH:-"$COVERAGE_REPORT_ROOT/tarpaulin-report.html"}"
COVERAGE_REPORT_DEPLOYMENT_ROOT="${COVERAGE_REPORT_DEPLOYMENT_ROOT:-"target/tarpaulin/pages-deployment/"}"
COVERAGE_REPORT_DEPLOYMENT_PATH="${COVERAGE_REPORT_DEPLOYMENT_PATH:-"coverage-report/$REF_NAME"}"

if [ ! -f "$COVERAGE_REPORT_FILE_PATH" ]; then
    echo "Coverage report not found at $COVERAGE_REPORT_FILE_PATH"
    exit 2
fi

echo "Creating deployment directory for ref '$REF_NAME' at '$COVERAGE_REPORT_DEPLOYMENT_PATH'"

if [ -n "$GITHUB_OUTPUT" ]; then
    echo "deployment_path=$COVERAGE_REPORT_DEPLOYMENT_PATH" >> "$GITHUB_OUTPUT"
fi

mkdir -p "$COVERAGE_REPORT_DEPLOYMENT_ROOT/$COVERAGE_REPORT_DEPLOYMENT_PATH"
cp "$COVERAGE_REPORT_FILE_PATH" "$COVERAGE_REPORT_DEPLOYMENT_ROOT/$COVERAGE_REPORT_DEPLOYMENT_PATH/index.html"
