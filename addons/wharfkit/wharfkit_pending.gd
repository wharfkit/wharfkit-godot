class_name WharfkitPending
extends Node

var _resolved: bool = false
var _connections: Array = []
var _continuation: Callable

func _wire_plugin(plugin: Object, done_name: String, failed_name: String, continuation: Callable) -> void:
	_continuation = continuation
	var on_done := Callable(self, "_on_plugin_done")
	var on_failed := Callable(self, "_on_plugin_failed")
	plugin.connect(done_name, on_done)
	plugin.connect(failed_name, on_failed)
	_connections.append([plugin, done_name, on_done])
	_connections.append([plugin, failed_name, on_failed])

func _on_plugin_done(response) -> void:
	if _resolved:
		return
	_resolved = true
	_disconnect_all()
	if _continuation.is_valid():
		_continuation.call(response)
	else:
		_emit_done(response)

func _on_plugin_failed(error) -> void:
	if _resolved:
		return
	_resolved = true
	_disconnect_all()
	_emit_failed(error)

func _emit_done(payload) -> void:
	_resolved = true
	_dispatch_done(payload)
	call_deferred("queue_free")

func _emit_failed(error) -> void:
	_resolved = true
	_dispatch_failed(error)
	call_deferred("queue_free")

func _dispatch_done(_payload) -> void:
	pass

func _dispatch_failed(_error) -> void:
	pass

func _disconnect_all() -> void:
	for entry in _connections:
		var emitter = entry[0]
		var sig: String = entry[1]
		var cb: Callable = entry[2]
		if emitter and emitter.is_connected(sig, cb):
			emitter.disconnect(sig, cb)
	_connections.clear()
