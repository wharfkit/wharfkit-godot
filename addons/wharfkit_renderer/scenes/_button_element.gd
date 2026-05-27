class_name WharfkitButtonElement
extends Control

signal user_action(response: WharfkitPromptResponse)

var _button: Button
var _initial_label: String = ""

func set_label(label: String) -> void:
	_initial_label = label
	if _button != null:
		_button.text = label

func _ready() -> void:
	_button = $Button
	if _initial_label != "":
		_button.text = _initial_label
	_button.pressed.connect(_on_pressed)

func _on_pressed() -> void:
	user_action.emit(_make_response())

func _make_response():
	assert(false, "WharfkitButtonElement subclass must override _make_response")
	return null

func set_element_data(_element) -> void:
	pass
