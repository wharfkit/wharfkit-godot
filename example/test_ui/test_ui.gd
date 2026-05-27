class_name TestUserInterface
extends WharfkitUserInterface

var prompt_calls: Array = []

func _prompt(args) -> void:
	prompt_calls.append(args)
	if args == null:
		prompt_done.emit(WharfkitPromptResponse.accepted())
		return
	for el in args.elements:
		if el.kind == 2:
			prompt_done.emit(WharfkitPromptResponse.button_pressed(String(el.id)))
			return
	prompt_done.emit(WharfkitPromptResponse.accepted())

func _on_login() -> void:
	pass

func _on_error(_err) -> void:
	pass
