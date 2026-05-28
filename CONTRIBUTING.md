# Contributing to wharfkit-godot

Thanks for your interest in contributing! `wharfkit-godot` is a Godot 4 addon that exposes the [`wharfkit-rs`](https://github.com/wharfkit/wharfkit-rs) Antelope SDK to GDScript via a Rust GDExtension cdylib.

## Prerequisites

- **Rust** matching the pinned channel in `rust-toolchain.toml` (currently `1.95.0`)
- **Godot 4.x** (tested against `4.6.x`)
- **Xcode 16.x** for iOS builds (macOS host)
- Optional: `cargo-ndk` + Android NDK for Android targets

## Building

```bash
make build          # core cdylib for macOS arm64
make build-all      # macOS arm64 + iOS device + iOS sim
```

Or invoke cargo directly:

```bash
cargo build --release --target aarch64-apple-darwin
```

See [`BUILD.md`](./BUILD.md) for per-target detail and the full Makefile target list.

## Cross-repo development

`wharfkit-godot` depends on Rust crates from several upstream repositories:

- [`antelope-client`](https://github.com/telosnetwork/antelope-rs) — Antelope primitives, maintained by the [Telos Network](https://www.telos.net/). Patches we need before they land upstream live in our fork at [`wharfkit/antelope-rs`](https://github.com/wharfkit/antelope-rs).
- [`wharfkit-rs`](https://github.com/wharfkit/wharfkit-rs) — the WharfKit Rust libraries (session, signing-request, buoy-client, common, abicache, contract).
- [`wharfkit-godot-types`](https://github.com/wharfkit/wharfkit-godot-types) — typed-view helpers shared between the core cdylib and plugin cdylibs.
- [`wharfkit-godot-wallet-plugin-anchor`](https://github.com/wharfkit/wharfkit-godot-wallet-plugin-anchor) — the Anchor wallet plugin cdylib (a separate cdylib so plugin authors don't have to depend on the core's Rust types).

For changes confined to this repo, the published crates.io versions of those dependencies are used automatically — nothing extra to set up.

For changes that span multiple repos, clone the upstream repos side-by-side and let Cargo's `[patch.crates-io]` mechanism resolve to your local checkouts:

```
your-projects/
├── antelope-rs/
├── wharfkit-rs/
├── wharfkit-godot-types/
├── wharfkit-godot-wallet-plugin-anchor/
└── wharfkit-godot/
```

The `[patch.crates-io]` section in `Cargo.toml` redirects the relevant deps to the sibling directories:

```toml
[patch.crates-io]
antelope-client = { path = "../antelope-rs/crates/antelope" }
wharfkit-common = { path = "../wharfkit-rs/crates/wharfkit-common" }
wharfkit-abicache = { path = "../wharfkit-rs/crates/wharfkit-abicache" }
wharfkit-contract = { path = "../wharfkit-rs/crates/wharfkit-contract" }
wharfkit-session = { path = "../wharfkit-rs/crates/wharfkit-session" }
wharfkit-signing-request = { path = "../wharfkit-rs/crates/wharfkit-signing-request" }
wharfkit-buoy-client = { path = "../wharfkit-rs/crates/wharfkit-buoy-client" }
wharfkit-godot-types = { path = "../wharfkit-godot-types" }
```

Edits in any of those repos are picked up by `cargo build` here immediately — no `cargo publish` in the loop. When your work spans multiple repos, open one PR per repo and cross-link them in the descriptions.

## Testing

```bash
make test           # cargo test --release --target aarch64-apple-darwin
make check          # fmt-check + clippy + test
make smoke          # headless mocked smoke test in Godot
```

Debug builds are currently blocked by an upstream interaction between rustc 1.95 and gdext 0.3 (see [`BUILD.md`](./BUILD.md) for context); `make test` pins `--release --target aarch64-apple-darwin` to work around it.

## Code style

- Rust: `cargo fmt` before submitting; `cargo clippy -- -D warnings` should be clean.
- GDScript: follow [Godot 4's official style guide](https://docs.godotengine.org/en/stable/tutorials/scripting/gdscript/gdscript_styleguide.html). This repo uses tabs for indentation and typed `var` / signal payloads where practical.

## Submitting changes

1. Fork and create a feature branch.
2. Make your changes; add or update tests.
3. Run `make check` and ensure it passes.
4. Open a PR with a clear description and links to any related issues.

For substantial changes (new APIs, breaking changes, architectural decisions), please open an issue first to discuss the approach.

## License

Contributions to this repository are licensed under [AGPL-3.0-or-later](./LICENSE). The AGPL is a strong copyleft license; consider its implications for any project that links against this code.
