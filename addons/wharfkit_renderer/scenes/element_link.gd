extends WharfkitButtonElement

var _link_id: String = ""
var _href: String = ""

func set_element_data(element) -> void:
	_link_id = String(element.id)
	_href = String(element.href)
	set_label(String(element.label))

func _on_pressed() -> void:
	if _href != "":
		OS.shell_open(_href)
	user_action.emit(_make_response())

func _make_response():
	return WharfkitPromptResponse.link_opened(_link_id)
