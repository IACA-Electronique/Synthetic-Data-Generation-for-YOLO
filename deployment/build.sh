#!/bin/bash

# Docker build script
# Don't run outside a Docker build !

set -e

SYSTEM="${1:-linux}"
ARCH=$(uname -m)

# ------------------------------------------------------------------------------------------------------------ FUNCTIONS

function build_for_linux() {
    local project_name
    if ! project_name=$(get_project_name) || [ -z "$project_name" ]; then
        echo "Failed to get project name"
        return 1
    fi

    if [ "$ARCH" == "x86_64" ]; then
        echo "Linux AMD64 building.."
        rustup target add x86_64-unknown-linux-musl
        cargo build --release --target x86_64-unknown-linux-musl --target-dir /build/output
        mv "/build/output/x86_64-unknown-linux-musl/release/${project_name}" /build/output/app
    elif [ "$ARCH" == "aarch64" ]; then
        echo "Linux ARM64 building.."
        apt-get install -y gcc-aarch64-linux-gnu
        rustup target add aarch64-unknown-linux-musl
        cargo build --release --target aarch64-unknown-linux-musl --target-dir /build/output
        mv "/build/output/aarch64-unknown-linux-musl/release/${project_name}" /build/output/app
    else
        echo "Build for linux with arch '$ARCH' not exists."
        return 1
    fi
}

function get_project_name() {
    cargo metadata --no-deps --format-version 1 | jq -r '.packages[0].name'
}

function install_require_package() {
    apt-get install -y build-essential cmake pkg-config jq
}
# ------------------------------------------------------------------------------------------------------------ MAIN

EXIT_STATUS=0

install_require_package

if [ "$SYSTEM" == "linux" ]; then
    build_for_linux
    EXIT_STATUS=$?
fi

exit $EXIT_STATUS

