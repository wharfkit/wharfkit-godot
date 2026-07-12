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

### Session persistence

`WharfkitSessionKit.restore()` lets a relaunched game sign without re-pairing
the wallet, given a dict shaped like this (built after a successful login and
saved by the game):

```gdscript
var persisted := {
    "anchor_channel": anchor.channel_snapshot(),
    "permission_level": session.permission_level,
    "chain_id": session.chain.chain_id(),
}
# game-provided storage, e.g. a save file or ConfigFile
my_store.save("wharfkit_session", persisted)
```

```gdscript
# next launch:
var persisted = my_store.load("wharfkit_session")
var session = kit.restore(persisted)
if session == null:
    # snapshot missing, malformed, or chain_id no longer matches a
    # configured chain — fall back to a normal kit.login()
    pass
```

`restore()` returns an unparented `WharfkitSession` (matching `login()`'s
`_finalize_login` result) or `null` if the snapshot can't be turned into a
usable session.

`kit.logout(session)` clears the wallet plugin's own persist file (written by
its `persist_channel()` helper, if used) but has no effect on a `persisted`
copy the game saved itself — the game is responsible for discarding that copy
too.

> **Security:** `anchor_channel` contains the request-key **private WIF in
> plaintext**. Persisting it — via the wallet plugin's built-in helpers or the
> game's own storage — is plaintext-at-rest. Anyone holding the WIF plus the
> channel URL can impersonate the paired game and push arbitrary signing
> requests to the user's Anchor wallet, though the user still has to approve
> each one in Anchor; it is **not** an account key and cannot itself sign
> chain transactions. Encrypt it or use an OS keychain if your threat model
> requires it.

See `example/` in the source repo for a complete sample app.
