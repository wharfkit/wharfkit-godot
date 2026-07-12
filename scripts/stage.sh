#!/usr/bin/env bash
#
# stage.sh — populate the example/addons/ mirror with the latest canonical
# addon source plus freshly-built cdylib binaries. Run after `cargo build`
# (or via `make stage`). For distribution-ready canonical addon trees, see
# `make release` / scripts/release.sh.

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TARGET="${ROOT}/target"
EXAMPLE_ADDONS="${ROOT}/example/addons"

# Mirror canonical addon source into example/addons/.
# Excludes lib/ (staged below) and *.uid (Godot regenerates per-project).
# Sibling plugin addons are NOT staged here — see BUILD.md.
for addon in wharfkit wharfkit_renderer; do
    mkdir -p "${EXAMPLE_ADDONS}/${addon}"
    rsync -a --delete \
        --exclude="lib/" \
        --exclude="*.uid" \
        "${ROOT}/addons/${addon}/" "${EXAMPLE_ADDONS}/${addon}/"
done
echo "Mirrored canonical addon source into example/addons/"

EXAMPLE_LIB="${EXAMPLE_ADDONS}/wharfkit/lib"
mkdir -p "${EXAMPLE_LIB}/macos-arm64" "${EXAMPLE_LIB}/ios" "${EXAMPLE_LIB}/ios-sim"

# Ad-hoc re-sign after copy: when cp overwrites a dylib that's currently
# mapped in another process (e.g. an open Godot editor), macOS's code-signing
# trust state desyncs and the next dlopen aborts with SIGKILL (Code Signature
# Invalid). `codesign --force --sign -` is a no-op identity sign that resets
# the state.
stage_pair() {
    local src="$1" sub="$2" name="$3"
    if [ -f "${src}" ]; then
        cp -f "${src}" "${EXAMPLE_LIB}/${sub}/"
        codesign --force --sign - "${EXAMPLE_LIB}/${sub}/$(basename "${src}")" 2>/dev/null || true
        echo "Staged ${name}"
    fi
}

if [ -f "${TARGET}/aarch64-apple-darwin/release/libwharfkit_godot.dylib" ]; then
    stage_pair "${TARGET}/aarch64-apple-darwin/release/libwharfkit_godot.dylib" \
        "macos-arm64" "macOS arm64 dylib"
elif [ -f "${TARGET}/release/libwharfkit_godot.dylib" ]; then
    stage_pair "${TARGET}/release/libwharfkit_godot.dylib" \
        "macos-arm64" "macOS arm64 dylib (host-default)"
fi

stage_pair "${TARGET}/aarch64-apple-ios/release/libwharfkit_godot.a" \
    "ios" "iOS arm64 device static lib"
stage_pair "${TARGET}/aarch64-apple-ios-sim/release/libwharfkit_godot.a" \
    "ios-sim" "iOS arm64 sim static lib"
