extends TestRunner

const TestUserInterface = preload("res://test_ui/test_ui.gd")
const WharfkitRendererScript = preload("res://addons/wharfkit_renderer/wharfkit_renderer.gd")

func _ready() -> void:
	await test_renderer_class_registered()
	await test_renderer_instantiation()
	await test_prompt_modal_renders_elements()
	await test_renderer_button_response()
	await test_renderer_close_response()
	await test_renderer_accept_response()
	await test_countdown_expires_locally()
	await test_test_ui_adapter_responds()
	if not _failed:
		print("All tests passed.")

func test_renderer_class_registered() -> void:
	_check(ClassDB.class_exists("WharfkitUserInterface"), "WharfkitUserInterface base in ClassDB")

func test_renderer_instantiation() -> void:
	var renderer = WharfkitRendererScript.new()
	add_child(renderer)
	await get_tree().process_frame
	_check(renderer != null, "WharfkitRenderer instantiable")
	_check(renderer.has_signal("prompt_done"), "prompt_done signal exposed via base class")
	_check(renderer.has_signal("prompt_failed"), "prompt_failed signal exposed via base class")
	renderer.queue_free()
	await get_tree().process_frame

func _build_args(elements: Array, title: String = "T", body: String = "B", optional: bool = true) -> WharfkitPromptArgs:
	var args := WharfkitPromptArgs.new()
	args.title = title
	args.body = body
	args.optional = optional
	var arr: Array[WharfkitPromptElement] = []
	for el in elements:
		arr.append(el)
	args.elements = arr
	return args

func test_prompt_modal_renders_elements() -> void:
	var renderer = WharfkitRendererScript.new()
	add_child(renderer)
	await get_tree().process_frame
	var args := _build_args([
		WharfkitPromptElement.button("manual", "Sign manually"),
		WharfkitPromptElement.link("launch", "esr://x", "Open Anchor"),
		WharfkitPromptElement.accept("OK"),
	])
	renderer._prompt(args)
	await get_tree().process_frame
	var modal := _find_modal(renderer)
	_check(modal != null, "modal instantiated under renderer CanvasLayer")
	if modal == null:
		renderer.queue_free()
		return
	_check(modal.element_count() == 3, "elements container has 3 children, got %d" % modal.element_count())
	modal.user_responded.emit(WharfkitPromptResponse.closed())
	await get_tree().process_frame
	renderer.queue_free()
	await get_tree().process_frame

func test_renderer_button_response() -> void:
	var renderer = WharfkitRendererScript.new()
	add_child(renderer)
	await get_tree().process_frame
	var args := _build_args([WharfkitPromptElement.button("manual", "Sign manually")])
	var got_response = [null]
	renderer.prompt_done.connect(func(r): got_response[0] = r, CONNECT_ONE_SHOT)
	renderer._prompt(args)
	await get_tree().process_frame
	var modal := _find_modal(renderer)
	if modal == null:
		_fail("button response: modal not found")
		renderer.queue_free()
		return
	var element_node: Node = modal.get_element(0)
	element_node.user_action.emit(WharfkitPromptResponse.button_pressed("manual"))
	await get_tree().process_frame
	_check(got_response[0] != null, "renderer emitted prompt_done")
	if got_response[0] != null:
		_check(got_response[0].kind == 3, "button response kind=ButtonPressed(3), got %d" % got_response[0].kind)
		_check(String(got_response[0].id) == "manual", "button response id=manual, got '%s'" % String(got_response[0].id))
	renderer.queue_free()
	await get_tree().process_frame

func test_renderer_close_response() -> void:
	var renderer = WharfkitRendererScript.new()
	add_child(renderer)
	await get_tree().process_frame
	var args := _build_args([WharfkitPromptElement.button("manual", "Sign manually")], "T", "B", true)
	var got_response = [null]
	renderer.prompt_done.connect(func(r): got_response[0] = r, CONNECT_ONE_SHOT)
	renderer._prompt(args)
	await get_tree().process_frame
	var modal := _find_modal(renderer)
	if modal == null:
		_fail("close response: modal not found")
		renderer.queue_free()
		return
	var close_btn: Button = modal.get_node("Panel/VBox/CloseButton")
	_check(close_btn.visible, "close button visible when optional=true")
	close_btn.pressed.emit()
	await get_tree().process_frame
	_check(got_response[0] != null, "renderer emitted prompt_done from close")
	if got_response[0] != null:
		_check(got_response[0].kind == 1, "close response kind=Closed(1), got %d" % got_response[0].kind)
	renderer.queue_free()
	await get_tree().process_frame

func test_renderer_accept_response() -> void:
	var renderer = WharfkitRendererScript.new()
	add_child(renderer)
	await get_tree().process_frame
	var args := _build_args([WharfkitPromptElement.accept("OK")])
	var got_response = [null]
	renderer.prompt_done.connect(func(r): got_response[0] = r, CONNECT_ONE_SHOT)
	renderer._prompt(args)
	await get_tree().process_frame
	var modal := _find_modal(renderer)
	if modal == null:
		_fail("accept response: modal not found")
		renderer.queue_free()
		return
	var element_node: Node = modal.get_element(0)
	var accept_button: Button = element_node.get_node("Button")
	accept_button.pressed.emit()
	await get_tree().process_frame
	_check(got_response[0] != null, "renderer emitted prompt_done from accept")
	if got_response[0] != null:
		_check(got_response[0].kind == 0, "accept response kind=Accepted(0), got %d" % got_response[0].kind)
	renderer.queue_free()
	await get_tree().process_frame

func test_countdown_expires_locally() -> void:
	var renderer = WharfkitRendererScript.new()
	add_child(renderer)
	await get_tree().process_frame
	var past_ms := int(Time.get_unix_time_from_system() * 1000.0) - 1000
	var args := _build_args([WharfkitPromptElement.countdown("expires", "Expires in", past_ms)])
	var got_response = [null]
	renderer.prompt_done.connect(func(r): got_response[0] = r, CONNECT_ONE_SHOT)
	renderer._prompt(args)
	await get_tree().process_frame
	await get_tree().process_frame
	_check(got_response[0] != null, "countdown produced response")
	if got_response[0] != null:
		_check(got_response[0].kind == 2, "countdown response kind=Expired(2), got %d" % got_response[0].kind)
	renderer.queue_free()
	await get_tree().process_frame

func test_test_ui_adapter_responds() -> void:
	var ui = TestUserInterface.new()
	add_child(ui)
	await get_tree().process_frame
	var args := _build_args([WharfkitPromptElement.button("manual", "Sign manually")])
	var got_response = [null]
	ui.prompt_done.connect(func(r): got_response[0] = r, CONNECT_ONE_SHOT)
	ui._prompt(args)
	await get_tree().process_frame
	_check(got_response[0] != null, "TestUserInterface emitted prompt_done")
	if got_response[0] != null:
		_check(got_response[0].kind == 3, "TestUserInterface kind=ButtonPressed(3), got %d" % got_response[0].kind)
		_check(String(got_response[0].id) == "manual", "TestUserInterface id=manual")
	_check(ui.prompt_calls.size() == 1, "TestUserInterface recorded the call")
	ui.queue_free()
	await get_tree().process_frame

func _find_modal(renderer: Node) -> Control:
	for child in renderer.get_children():
		if child is CanvasLayer:
			for grandchild in child.get_children():
				if grandchild is Control:
					return grandchild
	return null
