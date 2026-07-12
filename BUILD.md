# Building wharfkit-godot

Build instructions for the Rust cdylibs and the in-repo example project.

## Prerequisites

- **Rust 1.95.0** (pinned by `rust-toolchain.toml`)
- **Rust targets:** `aarch64-apple-darwin`, `aarch64-apple-ios`, `aarch64-apple-ios-sim`
- **Xcode 16.x** with Command Line Tools (for iOS targets)
- **Godot 4.6.3** at `/Applications/Godot.app` (override with `GODOT_BIN=...`)
- **Sibling repos** cloned next to this one if you want to build against local sources (see [`CONTRIBUTING.md`](./CONTRIBUTING.md))

Verify:

```bash
rustc --version            # 1.95.0
xcodebuild -version        # Xcode 16.x
/Applications/Godot.app/Contents/MacOS/Godot --version  # 4.6.3.stable.official
```

## Addon overview

This repo ships two Godot addons:

- `addons/wharfkit/` — core. The cdylib + GDScript orchestrator (`WharfkitSessionKit`, `WharfkitSession`, pending awaitables) + plugin-contract base classes. Required by every other addon.
- `addons/wharfkit_renderer/` — pure-GDScript default prompt UI. Required for the sample app and for any project that wants the built-in modal.

Wallet plugins, transact plugins, and alternative UI renderers ship as separate repos (e.g. [`wharfkit-godot-wallet-plugin-anchor`](https://github.com/wharfkit/wharfkit-godot-wallet-plugin-anchor)) with their own `addons/<plugin>/` distributions.

The core addon must load before any other WharfKit addon (Godot loads addons in lexical order, so `wharfkit/` sorts ahead of `wharfkit_*/` automatically).

## Configuration

Some scripts (notably `scripts/ios-deploy.sh`) need machine-specific values: iOS device IDs, app bundle id, etc. Copy the template and fill in your values:

```bash
cp .env.example .env
$EDITOR .env
```

`.env` is gitignored. The Makefile and scripts source it automatically when present. Anything set there can also be overridden by exporting the env var directly in your shell.

## Common workflow (Makefile)

```bash
make build          # core cdylib for macOS arm64
make stage          # build + populate example/addons/ for in-repo testing
make run            # launch the example app (runs main.tscn)
make editor         # open the example project in the Godot editor
make smoke          # headless mocked smoke test (no network)
make check          # fmt-check + clippy + test (release mode)
make release        # build all targets + populate canonical addons/*/lib/ for distribution
make clean          # remove target/ and example/addons/
```

Run `make` (no target) for the full target list.

> **Note:** debug builds are blocked by an upstream interaction between rustc 1.95 and gdext 0.3 (raw-pointer-cast lifetime extension previously accepted; see <https://github.com/rust-lang/rust/issues/141402>). All targets in the Makefile pin `--release --target aarch64-apple-darwin`.

## Building individual targets

For finer-grained builds, invoke cargo directly. All commands run from the repo root.

### macOS arm64 (host)

```bash
cargo build --release --target aarch64-apple-darwin
```

Output: `target/aarch64-apple-darwin/release/libwharfkit_godot.dylib` (~6 MB)

### iOS arm64 device

```bash
cargo build --release --target aarch64-apple-ios
```

Output: `target/aarch64-apple-ios/release/libwharfkit_godot.a` (~63 MB)

iOS uses a static library (`crate-type = ["staticlib", ...]` in `Cargo.toml`). Godot's iOS exporter links it into the host Xcode app and injects the `gdext_rust_init` call into the generated `dummy.cpp` so the linker pulls in the archive.

### iOS arm64 simulator

```bash
cargo build --release --target aarch64-apple-ios-sim
```

Output: `target/aarch64-apple-ios-sim/release/libwharfkit_godot.a` (~63 MB)

### Anchor plugin cdylib (sibling repo)

The Anchor plugin lives in `../wharfkit-godot-wallet-plugin-anchor/`. Build after the core (incremental):

```bash
cargo build --release --target aarch64-apple-darwin \
  --manifest-path ../wharfkit-godot-wallet-plugin-anchor/Cargo.toml
```

Its own `scripts/stage.sh` stages the resulting dylib into both this repo's example mirror and its own canonical addon `lib/`.

## Staging

`make stage` (= `./scripts/stage.sh`) populates `example/addons/`:

- Mirrors canonical `addons/wharfkit/` and `addons/wharfkit_renderer/` source via `rsync` (excludes `lib/` and `*.uid`).
- Copies built cdylibs from `target/<arch>/release/` into `example/addons/wharfkit/lib/<platform>/`.
- Plugin addons (e.g. `wharfkit_wallet_plugin_anchor`) are pulled into `example/addons/` via `make bootstrap` — downloads the latest release tarball from GitHub. Pin a specific version with `ANCHOR_VERSION=v0.2.0 make bootstrap`.
- For active local development against an unreleased sibling plugin, `make bootstrap` won't help (it only fetches published releases) and the plugin's own `scripts/stage.sh` only populates *its own* repo's `addons/…/lib/` — it does not write here. Copy the freshly built cdylib in yourself, then re-sign it so an in-flight editor doesn't reject it:

  ```bash
  make -C ../wharfkit-godot-wallet-plugin-anchor build
  cp -f ../wharfkit-godot-wallet-plugin-anchor/target/aarch64-apple-darwin/release/libwharfkit_godot_wallet_plugin_anchor.dylib \
        example/addons/wharfkit_wallet_plugin_anchor/lib/macos-arm64/
  codesign --force --sign - example/addons/wharfkit_wallet_plugin_anchor/lib/macos-arm64/libwharfkit_godot_wallet_plugin_anchor.dylib
  ```
- Re-signs each copied dylib (`codesign --force --sign -`) so an in-flight Godot editor doesn't reject the new file with `SIGKILL (Code Signature Invalid)`.

`make release` (= `./scripts/release.sh`) populates the canonical `addons/*/lib/` directories — for producing distribution-ready addon trees (zip → AssetLib, drop into another Godot project, etc.).

## Headless smoke

```bash
make smoke
```

Equivalent to:

```bash
/Applications/Godot.app/Contents/MacOS/Godot \
  --headless \
  --path example \
  res://tests/test_session_kit.tscn \
  --quit-after 60
```

Expected output (last lines):

```
All tests passed.
MOCKED SMOKE: PASS
```

The mocked smoke uses `example/test_wallet/` (synthetic `WharfkitWalletPlugin` subclass) and `example/test_ui/` (auto-respond `WharfkitUserInterface` subclass) — no Buoy, Anchor, or chain traffic.

For the chain-client integration smoke (queries `teamgreymass`'s balance on Jungle 4):

```bash
/Applications/Godot.app/Contents/MacOS/Godot \
  --headless \
  --path example \
  res://tests/test_chains.tscn \
  --quit-after 15
```

Expected output:

```
All tests passed.
```

## Live Anchor flow (manual)

The example app (`example/main.tscn`) drives a real Anchor login + transfer against Jungle 4. Open in the Godot editor (the prompt modal needs a viewport).

Prerequisites:

- **Anchor mobile** installed on a phone with an account on Jungle 4 (`73e4385a2708e6d7048834fbc1079f2fabb17b3c125b146af438971e90716c4d`).
- macOS arm64 builds of both cdylibs staged (`make stage` + the Anchor repo's stage script).

Click-through:

1. `make run` (or `make editor` + play the main scene).
2. The BalanceLabel shows `teamgreymass balance: X.XXXX EOS`, refreshing every 5s.
3. Click **Login with Anchor**. A modal opens with the ESR URI and a **Launch Anchor** button.
4. Click **Launch Anchor** (same-device) or copy/scan the URI (cross-device). Approve in Anchor.
5. The status updates to `Logged in as <actor>@active` and the timer switches to watching `<actor>`'s balance.
6. Click **Transfer 0.0001 EOS to teamgreymass**. Anchor prompts on the phone; approve.
7. The cdylib broadcasts via `send_transaction2` and emits `sign_done` with `transaction_id`.
8. Click **View transaction (...)** to open the explorer.

Failure modes:

- Close the modal mid-flow → `WharfkitErrorKind.UserClosed`.
- Wait past the 2-minute timeout → `WharfkitErrorKind.Expired`.
- Anchor rejects on the phone → `WharfkitErrorKind.UserRejected`.
- Chain rejects the transaction → `WharfkitErrorKind.Internal` carrying the chain's error message.

Adjust the constants at the top of `example/main.gd` (transfer destination, amount) to suit your test account.

## iOS device deploy

See `scripts/ios-deploy.sh` for a full pipeline that builds, stages, exports to Xcode, patches `Info.plist` (URL schemes, orientations), and installs/launches on a device via `xcrun devicectl`. Run `scripts/ios-deploy.sh --help` for usage.

The script requires:

- An Apple Developer signing identity in the keychain.
- A provisioning profile installed for the bundle id (default `com.wharfkit.example`; override via `IOS_BUNDLE_ID` in `.env`).
- A paired iOS device (`xcrun devicectl list devices`).
- `IOS_DEVICE_BUILD_ID` and `IOS_DEVICE_CTL_ID` set in `.env` (see [Configuration](#configuration)).

## Known notes

- **API/runtime version skew is expected.** gdext 0.3.x logs `API v4.4.stable.official, runtime v4.6.3.stable.official` at startup; the engine's safeguards handle the skew.
- **`crate-type` is `["staticlib", "cdylib", "rlib"]`** — staticlib for iOS, cdylib for macOS/desktop, rlib for `cargo test`.
- **`addons/*/lib/` and `example/addons/` are gitignored.** Run `make stage` (or `make release` for distribution) to populate them.
