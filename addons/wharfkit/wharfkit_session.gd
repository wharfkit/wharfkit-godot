class_name WharfkitSession
extends Node

var chain
var permission_level: Dictionary = {}
var _plugin: Object
var _ui
var _return_path: String = ""
var _transact_in_flight: bool = false

func configure(in_chain, perm: Dictionary, plugin: Object, ui, return_path: String = "") -> void:
	chain = in_chain
	permission_level = perm.duplicate() if perm != null else {}
	_plugin = plugin
	_ui = ui
	_return_path = return_path

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

	var resolved := _make_resolved_request(actions, opts)
	var ctx := _make_transact_ctx(resolved)

	pending._wire_plugin(_plugin, "sign_done", "sign_failed",
		Callable(self, "_finalize_transact").bind(pending, actions, opts))
	_plugin.call_deferred("call", "_sign", resolved, ctx)
	return pending

func _make_resolved_request(actions: Array, opts: Dictionary) -> Dictionary:
	return {
		"chain_id": chain.chain_id() if chain.has_method("chain_id") else "",
		"actions": actions,
		"signer": permission_level,
		"broadcast": opts.get("broadcast", false),
	}

func _make_transact_ctx(resolved_request: Dictionary) -> Dictionary:
	return {
		"chain": chain,
		"permission_level": permission_level,
		"return_path": _return_path,
		"resolved_request": resolved_request,
		"ui": _ui,
	}

func _on_transact_resolved(_x) -> void:
	_transact_in_flight = false

func _finalize_transact(response, pending: WharfkitTransactPending, actions: Array, opts: Dictionary) -> void:
	var signatures: Array = []
	var transaction_id := ""
	if response is Dictionary:
		signatures = response.get("signatures", [])
		transaction_id = String(response.get("transaction_id", ""))
	elif response != null and response.has_method("get"):
		signatures = response.get("signatures")
		var response_transaction_id = response.get("transaction_id")
		if response_transaction_id != null:
			transaction_id = String(response_transaction_id)
	var result := {
		"signatures": signatures,
		"actions": actions,
		"broadcast": opts.get("broadcast", false),
		"transaction_id": transaction_id,
	}
	pending._emit_done(result)
