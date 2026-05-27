# WharfKit Renderer addon

Default `WharfkitUserInterface` implementation in pure GDScript. Renders the
prompt modal (QR, link, button, countdown, accept, close) that wallet plugins
drive during login and transact.

## Requirements

- Godot 4.6.3 or newer.
- The `addons/wharfkit/` core addon installed and enabled.

## What this addon ships

- `wharfkit_renderer.gd` — `WharfkitRenderer` (extends `WharfkitUserInterface`).
  Attach as a child of your scene root; pass to
  `WharfkitSessionKit.configure({"ui": renderer, ...})`.
- `scenes/prompt_modal.tscn` — modal container.
- `scenes/element_*.tscn` — one scene per `PromptElement` variant.

## Install

1. Copy this addons folder into your Godot project's `addons/` directory.
2. Enable in Project Settings → Plugins → "WharfKit Renderer" → Enable.

## Customization

The renderer is pure GDScript with no native dependencies. Subclass
`WharfkitRenderer` (or copy + edit the scenes) to brand the modal. The
contract you must preserve is the `_prompt(args) -> void` method + the
paired `prompt_done(response)` / `prompt_failed(error)` signals.
