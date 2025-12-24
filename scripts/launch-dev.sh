#!/bin/bash
# Launch script for Pulito development/testing
# This script runs the built application from the debug directory

cd "$(dirname "$0")/.."
exec ./src-tauri/target/debug/pulito "$@"
