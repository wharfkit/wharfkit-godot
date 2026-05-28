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

mkdir -p "${ADDON_LIB}/macos-arm64" "${ADDON_LIB}/ios" "${ADDON_LIB}/ios-sim"

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
