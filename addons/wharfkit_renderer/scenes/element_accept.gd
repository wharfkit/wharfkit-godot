extends WharfkitButtonElement

func _ready() -> void:
	if _initial_label == "":
		set_label("Accept")
	super()

func set_element_data(element) -> void:
	var text := String(element.label)
	if text != "":
		set_label(text)

func _make_response():
	return WharfkitPromptResponse.accepted()
