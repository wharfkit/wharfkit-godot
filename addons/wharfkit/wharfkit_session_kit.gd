class_name WharfkitSessionKit
extends Node

var _config: Dictionary = {}
var _login_in_flight: bool = false

func configure(opts: Dictionary) -> void:
	_config = opts.duplicate()
	var ui = _config.get("ui")
	if ui is Node and ui.get_parent() == null:
		add_child(ui)
	var plugins = _config.get("wallet_plugins", [])
	for plugin in plugins:
		if plugin is Node and plugin.get_parent() == null:
			add_child(plugin)

func login(_opts: Dictionary = {}) -> WharfkitLoginPending:
	var pending := WharfkitLoginPending.new()
	add_child(pending)

	if _login_in_flight:
		pending.call_deferred("_emit_failed", WharfkitError.internal("login already in flight"))
		return pending

	var chains: Array = _config.get("chains", [])
	if chains.is_empty():
		pending._emit_failed(WharfkitError.internal("no chains configured"))
		return pending

	var plugins: Array = _config.get("wallet_plugins", [])
	if plugins.is_empty():
		pending._emit_failed(WharfkitError.internal("no wallet plugins configured"))
		return pending

	_login_in_flight = true
	pending.login_done.connect(_on_login_resolved)
	pending.login_failed.connect(_on_login_resolved)

	var plugin = plugins[0]
	var chain = chains[0]
	var ctx := _make_login_ctx(chain)

	pending._wire_plugin(plugin, "login_done", "login_failed",
		Callable(self, "_finalize_login").bind(pending, chain, plugin))
	plugin.call_deferred("call", "_login", ctx)
	return pending

func _on_login_resolved(_x) -> void:
	_login_in_flight = false

func _make_login_ctx(chain) -> Dictionary:
	return {
		"chain": {
			"id": chain.chain_id() if chain.has_method("chain_id") else "",
			"url": chain.url_value() if chain.has_method("url_value") else "",
		},
		"app_name": _config.get("app_name", "WharfKit Godot"),
		"return_path": _config.get("return_path", ""),
	}

func _finalize_login(response, pending: WharfkitLoginPending, chain, plugin) -> void:
	var permission: Dictionary = {}
	if response is Dictionary:
		permission = response.get("permission_level", {})
	elif response != null and response.has_method("get"):
		permission = response.get("permission_level")

	var session := WharfkitSession.new()
	session.configure(chain, permission, plugin, _config.get("ui"), _config.get("return_path", ""))
	pending._emit_done(session)
