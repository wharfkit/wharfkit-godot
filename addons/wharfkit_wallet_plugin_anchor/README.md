# WharfKit Anchor wallet plugin

GDExtension wallet plugin for [Anchor](https://greymass.com/anchor). Connects
your Godot game to the Anchor mobile and desktop wallets via the Buoy relay
(defaults to `https://cb.anchor.link`).

## Requirements

- Godot 4.6.3 or newer.
- The `addons/wharfkit/` core addon installed and enabled.

## What this addon ships

Godot classes registered into `ClassDB`:

- `WharfkitWalletPluginAnchor` — Anchor wallet plugin. Pass to
  `WharfkitSessionKit.configure({"wallet_plugins": [...]})`. Optional
  `set_buoy_relay(url)` for routing through a custom relay.
- `WharfkitEosioToken` — typed action factory for the `eosio.token` contract
  (`transfer(from, to, quantity, memo, actor, permission) -> Dictionary`).

## Install

1. Copy this addons folder into your Godot project's `addons/` directory.
2. Stage the platform-specific cdylib into `lib/<platform>/`. Build instructions
   in `BUILD.md`.
3. Enable in Project Settings → Plugins → "WharfKit Anchor Wallet Plugin" → Enable.

The core `addons/wharfkit/` addon must be enabled and loaded before this one.

## Usage

```gdscript
var anchor := WharfkitWalletPluginAnchor.new()
# Optional: route Buoy traffic through a custom relay.
# anchor.set_buoy_relay("http://127.0.0.1:8080")

var kit := WharfkitSessionKit.new()
add_child(kit)
kit.configure({
    "app_name": "My Game",
    "chains": [WharfkitChains.jungle4()],
    "ui": WharfkitRenderer.new(),
    "wallet_plugins": [anchor],
})
var pending = kit.login()
var session = await pending.login_done
```

## Supported scope

Transactions involving the `eosio.token` contract are supported out of the box.
Other contracts and action types require additional ABI support.
