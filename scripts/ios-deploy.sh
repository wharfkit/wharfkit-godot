#!/usr/bin/env bash
#
# ios-deploy.sh — full iOS device deploy pipeline for the wharfkit-godot
# example app. Builds cdylibs → stages → exports → patches Info.plist →
# xcodebuilds → installs on device → launches.
#
# Usage:
#   scripts/ios-deploy.sh                # full pipeline
#   scripts/ios-deploy.sh --skip-build   # skip cargo build (use existing staged)
#   scripts/ios-deploy.sh --no-launch    # build+export+xcodebuild, skip install/launch
#   scripts/ios-deploy.sh --no-install   # build+export+xcodebuild, skip install only (no launch)
#
# Required environment (set in .env or your shell — see .env.example):
#   IOS_DEVICE_BUILD_ID    Xcode build destination id
#   IOS_DEVICE_CTL_ID      devicectl device id
#
# Optional environment (with sensible defaults):
#   IOS_BUNDLE_ID          App bundle id (default: com.wharfkit.example)
#   IOS_PRESET             Godot export preset name (default: iOS-device)
#   IOS_CONFIG             xcodebuild -configuration (default: Debug)
#   GODOT_BIN              Path to Godot binary (default: /Applications/Godot.app/Contents/MacOS/Godot)
#
# Prerequisites:
#   - Apple Developer signing identity in keychain (`security find-identity -p codesigning`)
#   - Provisioning profile for IOS_BUNDLE_ID installed in ~/Library/Developer/Xcode/UserData/Provisioning Profiles/
#   - Device paired with Xcode (`xcrun devicectl list devices`)
#   - cdylibs built for aarch64-apple-ios already, OR will be built fresh
#
# What this script automates that Godot's preset can't:
#   - Info.plist additions Godot doesn't expose via preset:
#     * LSApplicationQueriesSchemes — iOS silently blocks UIApplication.open()
#       for undeclared schemes; without esr/anchor here, OS.shell_open("esr://…")
#       is a no-op.
#     * CFBundleURLTypes — registers wharfkit:// as a return-path scheme so
#       Anchor's redirect lands back in our app.
#     * UISupportedInterfaceOrientations — portrait-only on phone (Anchor flows
#       are designed portrait).

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
WORKSPACE="$(cd "${ROOT}/.." && pwd)"

# Load .env if present so machine-specific overrides apply.
if [ -f "${ROOT}/.env" ]; then
    set -a
    # shellcheck disable=SC1091
    . "${ROOT}/.env"
    set +a
fi

CORE_MANIFEST="${ROOT}/Cargo.toml"
ANCHOR_MANIFEST="${WORKSPACE}/wharfkit-godot-wallet-plugin-anchor/Cargo.toml"
STAGE_SH="${ROOT}/scripts/stage.sh"
EXAMPLE_PROJECT="${ROOT}/example"
BUILD_DIR="${ROOT}/build/ios"
DERIVED_DATA="${ROOT}/build/ios/DerivedData"
XCODE_PROJECT_NAME="wharfkit-godot-example"

: "${IOS_DEVICE_BUILD_ID:?IOS_DEVICE_BUILD_ID must be set (in .env or env). See .env.example.}"
: "${IOS_DEVICE_CTL_ID:?IOS_DEVICE_CTL_ID must be set (in .env or env). See .env.example.}"
IOS_BUNDLE_ID="${IOS_BUNDLE_ID:-com.wharfkit.example}"
IOS_PRESET="${IOS_PRESET:-iOS-device}"
IOS_CONFIG="${IOS_CONFIG:-Debug}"
GODOT_BIN="${GODOT_BIN:-/Applications/Godot.app/Contents/MacOS/Godot}"

SKIP_BUILD=0
NO_LAUNCH=0
NO_INSTALL=0
for arg in "$@"; do
    case "${arg}" in
        --skip-build) SKIP_BUILD=1 ;;
        --no-launch)  NO_LAUNCH=1 ;;
        --no-install) NO_INSTALL=1; NO_LAUNCH=1 ;;
        -h|--help)
            sed -n '2,/^$/p' "${BASH_SOURCE[0]}" | sed 's/^# \{0,1\}//'
            exit 0
            ;;
        *)
            echo "ERROR: unknown arg: ${arg}" >&2
            exit 1
            ;;
    esac
done

if [ ! -x "${GODOT_BIN}" ]; then
    echo "ERROR: Godot binary not found at ${GODOT_BIN} (override with GODOT_BIN=...)" >&2
    exit 1
fi

# --- 1. Build Rust cdylibs for iOS device ---------------------------------

if [ "${SKIP_BUILD}" -eq 0 ]; then
    echo "==> building wharfkit-godot (core) for aarch64-apple-ios"
    cargo build --manifest-path "${CORE_MANIFEST}" --target aarch64-apple-ios --release
    echo "==> building wharfkit-godot-wallet-plugin-anchor for aarch64-apple-ios"
    cargo build --manifest-path "${ANCHOR_MANIFEST}" --target aarch64-apple-ios --release
fi

# --- 2. Stage cdylibs (mostly a no-op for iOS .a paths, but keeps parity) -

echo "==> staging cdylibs"
"${STAGE_SH}"

# --- 3. Godot iOS export --------------------------------------------------

mkdir -p "${BUILD_DIR}"
XCODE_PROJECT="${BUILD_DIR}/${XCODE_PROJECT_NAME}.xcodeproj"
echo "==> Godot --export-debug ${IOS_PRESET} → ${XCODE_PROJECT}"
"${GODOT_BIN}" --headless --path "${EXAMPLE_PROJECT}" --export-debug "${IOS_PRESET}" "${XCODE_PROJECT}"

# --- 4. Patch Info.plist --------------------------------------------------
#
# Godot's iOS exporter does not expose LSApplicationQueriesSchemes,
# CFBundleURLTypes, or per-orientation UISupportedInterfaceOrientations
# via the export preset. Patch them post-export.

INFO_PLIST="$(find "${BUILD_DIR}/${XCODE_PROJECT_NAME}" -maxdepth 2 -name "*-Info.plist" | head -1)"
if [ -z "${INFO_PLIST}" ] || [ ! -f "${INFO_PLIST}" ]; then
    echo "ERROR: Info.plist not found under ${BUILD_DIR}/${XCODE_PROJECT_NAME}" >&2
    echo "       (Godot export step may have failed; check output above)" >&2
    exit 1
fi
echo "==> patching ${INFO_PLIST}"

# Idempotent: delete-then-add so reruns don't accumulate duplicates.
pb() { /usr/libexec/PlistBuddy -c "$1" "${INFO_PLIST}" 2>/dev/null || true; }
pbreq() { /usr/libexec/PlistBuddy -c "$1" "${INFO_PLIST}"; }

# LSApplicationQueriesSchemes = ["esr", "anchor"]
pb "Delete :LSApplicationQueriesSchemes"
pbreq "Add :LSApplicationQueriesSchemes array"
pbreq "Add :LSApplicationQueriesSchemes:0 string esr"
pbreq "Add :LSApplicationQueriesSchemes:1 string anchor"

# CFBundleURLTypes = [{CFBundleURLName: <bundle-id>, CFBundleURLSchemes: ["wharfkit"]}]
pb "Delete :CFBundleURLTypes"
pbreq "Add :CFBundleURLTypes array"
pbreq "Add :CFBundleURLTypes:0 dict"
pbreq "Add :CFBundleURLTypes:0:CFBundleURLName string ${IOS_BUNDLE_ID}"
pbreq "Add :CFBundleURLTypes:0:CFBundleURLSchemes array"
pbreq "Add :CFBundleURLTypes:0:CFBundleURLSchemes:0 string wharfkit"

# UISupportedInterfaceOrientations = [UIInterfaceOrientationPortrait]
pb "Delete :UISupportedInterfaceOrientations"
pbreq "Add :UISupportedInterfaceOrientations array"
pbreq "Add :UISupportedInterfaceOrientations:0 string UIInterfaceOrientationPortrait"

# --- 5. xcodebuild --------------------------------------------------------

echo "==> xcodebuild -configuration ${IOS_CONFIG} for device ${IOS_DEVICE_BUILD_ID}"
xcodebuild \
    -project "${XCODE_PROJECT}" \
    -scheme "${XCODE_PROJECT_NAME}" \
    -configuration "${IOS_CONFIG}" \
    -destination "platform=iOS,id=${IOS_DEVICE_BUILD_ID}" \
    -allowProvisioningUpdates \
    -derivedDataPath "${DERIVED_DATA}" \
    build

APP_BUNDLE="${DERIVED_DATA}/Build/Products/${IOS_CONFIG}-iphoneos/${XCODE_PROJECT_NAME}.app"
if [ ! -d "${APP_BUNDLE}" ]; then
    echo "ERROR: built app bundle not found at ${APP_BUNDLE}" >&2
    exit 1
fi

# --- 6. Install + launch via devicectl ------------------------------------

if [ "${NO_INSTALL}" -eq 1 ]; then
    echo "==> --no-install set; built ${APP_BUNDLE} but not installing"
    exit 0
fi

echo "==> xcrun devicectl device install app --device ${IOS_DEVICE_CTL_ID}"
xcrun devicectl device install app --device "${IOS_DEVICE_CTL_ID}" "${APP_BUNDLE}"

if [ "${NO_LAUNCH}" -eq 1 ]; then
    echo "==> --no-launch set; installed but not launching"
    exit 0
fi

echo "==> xcrun devicectl device process launch ${IOS_BUNDLE_ID}"
xcrun devicectl device process launch --device "${IOS_DEVICE_CTL_ID}" "${IOS_BUNDLE_ID}"

echo "==> Done. Tail device logs with:"
echo "    xcrun devicectl device syslog stream --device ${IOS_DEVICE_CTL_ID} --predicate 'subsystem == \"${IOS_BUNDLE_ID}\"'"
