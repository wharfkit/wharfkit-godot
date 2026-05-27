class_name WharfkitSession
extends Node

var chain
var permission_level: Dictionary = {}
var _plugin: Object
var _ui
var _transact_in_flight: bool = false

func configure(in_chain, perm: Dictionary, plugin: Object, ui) -> void:
	chain = in_chain
	permission_level = perm.duplicate() if perm != null else {}
	_plugin = plugin
	_ui = ui

func transact(args: Dictionary, opts: Dictionary = {}) -> WharfkitTransactPending:
	var pending := WharfkitTransactPending.new()
	add_child(pending)

	if _transact_in_flight:
		pending.call_deferred("_emit_failed", WharfkitError.internal("transact already in flight"))
		return pending

	var actions: Array = args.get("actions", [])
	if actions.is_empty():
		pending._emit_failed(WharfkitError.internal("transact: no actions"))
		return pending

	_transact_in_flight = true
	pending.transact_done.connect(_on_transact_resolved)
	pending.transact_failed.connect(_on_transact_resolved)

	var resolved := {
		"chain_id": chain.chain_id() if chain.has_method("chain_id") else "",
		"actions": actions,
		"signer": permission_level,
		"broadcast": opts.get("broadcast", false),
	}
	var ctx := {
		"chain": chain,
		"permission_level": permission_level,
	}

	pending._wire_plugin(_plugin, "sign_done", "sign_failed",
		Callable(self, "_finalize_transact").bind(pending, actions, opts))
	_plugin.call_deferred("call", "_sign", resolved, ctx)
	return pending

func _on_transact_resolved(_x) -> void:
	_transact_in_flight = false

func _finalize_transact(response, pending: WharfkitTransactPending, actions: Array, opts: Dictionary) -> void:
	var signatures: Array = []
	if response is Dictionary:
		signatures = response.get("signatures", [])
	elif response != null and response.has_method("get"):
		signatures = response.get("signatures")
	var result := {
		"signatures": signatures,
		"actions": actions,
		"broadcast": opts.get("broadcast", false),
		"transaction_id": "",
	}
	pending._emit_done(result)
