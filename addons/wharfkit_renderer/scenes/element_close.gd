extends WharfkitButtonElement

func _ready() -> void:
	set_label("Close")
	super()

func _make_response():
	return WharfkitPromptResponse.closed()
