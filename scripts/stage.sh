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

ANCHOR_ROOT="$(cd "${ROOT}/../wharfkit-godot-wallet-plugin-anchor" 2>/dev/null && pwd)" || ANCHOR_ROOT=""

# Mirror canonical addon source into example/addons/.
# Excludes lib/ (staged below) and *.uid (Godot regenerates per-project).
for addon in wharfkit wharfkit_renderer wharfkit_wallet_plugin_anchor; do
    mkdir -p "${EXAMPLE_ADDONS}/${addon}"
    rsync -a --delete \
        --exclude="lib/" \
        --exclude="*.uid" \
        "${ROOT}/addons/${addon}/" "${EXAMPLE_ADDONS}/${addon}/"
done
echo "Mirrored canonical addon source into example/addons/"

EXAMPLE_LIB="${EXAMPLE_ADDONS}/wharfkit/lib"
ANCHOR_EXAMPLE_LIB="${EXAMPLE_ADDONS}/wharfkit_wallet_plugin_anchor/lib"
mkdir -p "${EXAMPLE_LIB}/macos-arm64" "${EXAMPLE_LIB}/ios" "${EXAMPLE_LIB}/ios-sim"
mkdir -p "${ANCHOR_EXAMPLE_LIB}/macos-arm64" "${ANCHOR_EXAMPLE_LIB}/ios" "${ANCHOR_EXAMPLE_LIB}/ios-sim"

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

if [ -n "${ANCHOR_ROOT}" ]; then
    ANCHOR_DYLIB="${ANCHOR_ROOT}/target/aarch64-apple-darwin/release/libwharfkit_godot_wallet_plugin_anchor.dylib"
    if [ ! -f "${ANCHOR_DYLIB}" ]; then
        ANCHOR_DYLIB="${ANCHOR_ROOT}/target/release/libwharfkit_godot_wallet_plugin_anchor.dylib"
    fi
    if [ -f "${ANCHOR_DYLIB}" ]; then
        cp -f "${ANCHOR_DYLIB}" "${ANCHOR_EXAMPLE_LIB}/macos-arm64/"
        codesign --force --sign - "${ANCHOR_EXAMPLE_LIB}/macos-arm64/$(basename "${ANCHOR_DYLIB}")" 2>/dev/null || true
        echo "Staged Anchor wallet plugin dylib"
    fi

    ANCHOR_IOS_LIB="${ANCHOR_ROOT}/target/aarch64-apple-ios/release/libwharfkit_godot_wallet_plugin_anchor.a"
    if [ -f "${ANCHOR_IOS_LIB}" ]; then
        cp -f "${ANCHOR_IOS_LIB}" "${ANCHOR_EXAMPLE_LIB}/ios/"
        echo "Staged Anchor wallet plugin iOS arm64 device static lib"
    fi
    ANCHOR_IOSSIM_LIB="${ANCHOR_ROOT}/target/aarch64-apple-ios-sim/release/libwharfkit_godot_wallet_plugin_anchor.a"
    if [ -f "${ANCHOR_IOSSIM_LIB}" ]; then
        cp -f "${ANCHOR_IOSSIM_LIB}" "${ANCHOR_EXAMPLE_LIB}/ios-sim/"
        echo "Staged Anchor wallet plugin iOS arm64 sim static lib"
    fi
fi
