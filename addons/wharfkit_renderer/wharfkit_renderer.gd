class_name WharfkitRenderer
extends WharfkitUserInterface

const PROMPT_MODAL_SCENE: PackedScene = preload("res://addons/wharfkit_renderer/scenes/prompt_modal.tscn")

var _modal_layer: CanvasLayer

func _ready() -> void:
	_modal_layer = CanvasLayer.new()
	_modal_layer.layer = 100
	add_child(_modal_layer)

func _on_login() -> void:
	pass

func _on_login_complete() -> void:
	pass

func _on_transact() -> void:
	pass

func _on_transact_complete() -> void:
	pass

func _on_sign() -> void:
	pass

func _on_sign_complete() -> void:
	pass

func _on_broadcast() -> void:
	pass

func _on_broadcast_complete() -> void:
	pass

func _on_error(error: WharfkitError) -> void:
	if error == null:
		return
	push_warning("WharfKit error: [%d] %s" % [error.kind, error.message])

func _status(_message: String) -> void:
	pass

func _login(_ctx) -> void:
	prompt_failed.emit(WharfkitError.internal(
		"WharfkitRenderer._login is not implemented in Slice 2"
	))

func _prompt(args) -> void:
	if args == null:
		prompt_failed.emit(WharfkitError.internal("WharfkitRenderer._prompt received null args"))
		return
	var modal := PROMPT_MODAL_SCENE.instantiate()
	_modal_layer.add_child(modal)
	modal.populate(args.elements, String(args.title), String(args.body), args.optional)
	var response = await modal.user_responded
	modal.queue_free()
	prompt_done.emit(response)

func _close_prompt() -> void:
	for child in _modal_layer.get_children():
		if child is WharfkitPromptModal:
			child.user_responded.emit(WharfkitPromptResponse.closed())
			child.queue_free()
