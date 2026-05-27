extends WharfkitButtonElement

var _button_id: String = ""

func set_element_data(element) -> void:
	_button_id = String(element.id)
	set_label(String(element.label))

func _make_response():
	return WharfkitPromptResponse.button_pressed(_button_id)
