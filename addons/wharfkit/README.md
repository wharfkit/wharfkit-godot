# WharfKit core addon

Core addon for WharfKit-for-Godot. Hosts the shared runtime, plugin-contract
base classes, and the GDScript orchestrator that drives login and transact.
Required by every other WharfKit addon.

## Requirements

- Godot 4.6.3 or newer.
- macOS arm64 or iOS arm64 (device or simulator).

## What this addon ships

Godot classes registered into `ClassDB`:

- `WharfkitRuntime` — engine singleton powering background work.
- `WharfkitWalletPlugin`, `WharfkitUserInterface`, `WharfkitDeepLinkBridge` —
  base classes for plugin and UI authors.
- `WharfkitError`, `WharfkitErrorKind` — uniform error type.
- `WharfkitChains`, `WharfkitChainDefinition` — built-in chain registry
  (Jungle 4, EOS, WAX, Telos, Vaulta).
- `WharfkitContractKit`, `WharfkitContract`, `WharfkitTable`,
  `WharfkitName`, `WharfkitAsset`, `WharfkitSymbol`, `WharfkitChecksum256` —
  typed wrappers around Antelope chain primitives.
- `WharfkitPromptArgs`, `WharfkitPromptResponse`, `WharfkitPromptElement`,
  `WharfkitPromptHandle`, `WharfkitLoginContext`, `WharfkitTransactContext` —
  plugin-contract payload types.

GDScript classes:

- `WharfkitSessionKit` — top-level orchestrator. Holds wallet plugins, UI,
  and chain list; produces sessions via `login()`.
- `WharfkitSession` — logged-in handle; produces transactions via `transact()`.
- `WharfkitLoginPending`, `WharfkitTransactPending` — per-call awaitable
  objects exposing `<verb>_done` / `<verb>_failed` signals.

## Install

1. Copy this addons folder into your Godot project's `addons/` directory.
2. Stage the platform-specific cdylib into `addons/wharfkit/lib/<platform>/`.
   Build instructions in `BUILD.md`.
3. Enable in Project Settings → Plugins → "WharfKit" → Enable.

This addon must load before any other WharfKit addon. Godot loads addons in
lexical order, so `wharfkit/` sorts ahead of `wharfkit_*/` automatically.

## Usage

```gdscript
var kit := WharfkitSessionKit.new()
add_child(kit)
kit.configure({
    "app_name": "My Game",
    "chains": [WharfkitChains.jungle4()],
    "ui": WharfkitRenderer.new(),
    "wallet_plugins": [WharfkitWalletPluginAnchor.new()],
})

var pending = kit.login()
var session = await pending.login_done
print("Logged in as: ", session.permission_level)
```

See `example/` in the source repo for a complete sample app.
