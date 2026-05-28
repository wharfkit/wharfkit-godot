#!/usr/bin/env bash
#
# release.sh — populate the canonical addons/*/lib/ directories with built
# binaries, producing distribution-ready addon trees that can be zipped and
# uploaded to AssetLib or copied into other Godot projects.
#
# Run `make build-all` first (or via `make release`). For in-repo example
# testing, use `make stage` / scripts/stage.sh instead.

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TARGET="${ROOT}/target"
ADDON_LIB="${ROOT}/addons/wharfkit/lib"
ANCHOR_ROOT="$(cd "${ROOT}/../wharfkit-godot-wallet-plugin-anchor" 2>/dev/null && pwd)" || ANCHOR_ROOT=""
ANCHOR_ADDON_LIB="${ROOT}/addons/wharfkit_wallet_plugin_anchor/lib"

mkdir -p "${ADDON_LIB}/macos-arm64" "${ADDON_LIB}/ios" "${ADDON_LIB}/ios-sim"
mkdir -p "${ANCHOR_ADDON_LIB}/macos-arm64" "${ANCHOR_ADDON_LIB}/ios" "${ANCHOR_ADDON_LIB}/ios-sim"

stage_pair() {
    local src="$1" sub="$2" name="$3"
    if [ -f "${src}" ]; then
        cp -f "${src}" "${ADDON_LIB}/${sub}/"
        codesign --force --sign - "${ADDON_LIB}/${sub}/$(basename "${src}")" 2>/dev/null || true
        echo "Staged ${name}"
    else
        echo "WARN: missing ${src}" >&2
    fi
}

stage_pair "${TARGET}/aarch64-apple-darwin/release/libwharfkit_godot.dylib" \
    "macos-arm64" "macOS arm64 dylib"
stage_pair "${TARGET}/aarch64-apple-ios/release/libwharfkit_godot.a" \
    "ios" "iOS arm64 device static lib"
stage_pair "${TARGET}/aarch64-apple-ios-sim/release/libwharfkit_godot.a" \
    "ios-sim" "iOS arm64 sim static lib"

if [ -n "${ANCHOR_ROOT}" ]; then
    ANCHOR_DYLIB="${ANCHOR_ROOT}/target/aarch64-apple-darwin/release/libwharfkit_godot_wallet_plugin_anchor.dylib"
    if [ -f "${ANCHOR_DYLIB}" ]; then
        cp -f "${ANCHOR_DYLIB}" "${ANCHOR_ADDON_LIB}/macos-arm64/"
        codesign --force --sign - "${ANCHOR_ADDON_LIB}/macos-arm64/$(basename "${ANCHOR_DYLIB}")" 2>/dev/null || true
        echo "Staged Anchor wallet plugin dylib"
    fi

    ANCHOR_IOS_LIB="${ANCHOR_ROOT}/target/aarch64-apple-ios/release/libwharfkit_godot_wallet_plugin_anchor.a"
    if [ -f "${ANCHOR_IOS_LIB}" ]; then
        cp -f "${ANCHOR_IOS_LIB}" "${ANCHOR_ADDON_LIB}/ios/"
        echo "Staged Anchor wallet plugin iOS arm64 device static lib"
    fi
    ANCHOR_IOSSIM_LIB="${ANCHOR_ROOT}/target/aarch64-apple-ios-sim/release/libwharfkit_godot_wallet_plugin_anchor.a"
    if [ -f "${ANCHOR_IOSSIM_LIB}" ]; then
        cp -f "${ANCHOR_IOSSIM_LIB}" "${ANCHOR_ADDON_LIB}/ios-sim/"
        echo "Staged Anchor wallet plugin iOS arm64 sim static lib"
    fi
fi
