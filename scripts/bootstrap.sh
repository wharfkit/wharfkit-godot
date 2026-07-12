#!/usr/bin/env bash
# Download + extract the Anchor release tarball into example/addons/ (see BUILD.md).
set -euo pipefail

ANCHOR_VERSION="${ANCHOR_VERSION:-v0.1.0}"
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
EXAMPLE_ADDONS="${ROOT}/example/addons"

URL="https://github.com/wharfkit/wharfkit-godot-wallet-plugin-anchor/releases/download/${ANCHOR_VERSION}/wharfkit_wallet_plugin_anchor-${ANCHOR_VERSION}.tar.gz"

mkdir -p "${EXAMPLE_ADDONS}"

echo "Downloading Anchor ${ANCHOR_VERSION} tarball..."
curl -fsSL "${URL}" | tar -xz -C "${EXAMPLE_ADDONS}"
echo "Bootstrapped wharfkit_wallet_plugin_anchor ${ANCHOR_VERSION} into example/addons/"
