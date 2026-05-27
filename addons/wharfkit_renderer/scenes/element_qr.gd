extends Control

signal user_action(response: WharfkitPromptResponse)

var _data: String = ""
var _label: Label

func set_element_data(element) -> void:
	_data = String(element.data)
	if _label != null:
		_label.text = _display_text()

func _ready() -> void:
	_label = $Label
	_label.text = _display_text()

func _display_text() -> String:
	return "QR: %s" % _data
