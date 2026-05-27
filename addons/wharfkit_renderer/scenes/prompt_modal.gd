class_name WharfkitPromptModal
extends Control

signal user_responded(response: WharfkitPromptResponse)

const SCENES_BY_KIND := {
	0: preload("res://addons/wharfkit_renderer/scenes/element_qr.tscn"),
	1: preload("res://addons/wharfkit_renderer/scenes/element_link.tscn"),
	2: preload("res://addons/wharfkit_renderer/scenes/element_button.tscn"),
	3: preload("res://addons/wharfkit_renderer/scenes/element_countdown.tscn"),
	4: preload("res://addons/wharfkit_renderer/scenes/element_accept.tscn"),
	5: preload("res://addons/wharfkit_renderer/scenes/element_close.tscn"),
}

@onready var title_label: Label = $Panel/VBox/Title
@onready var body_label: Label = $Panel/VBox/Body
@onready var elements_container: VBoxContainer = $Panel/VBox/Elements
@onready var close_button: Button = $Panel/VBox/CloseButton

func _ready() -> void:
	close_button.pressed.connect(_on_close_pressed)

func populate(elements, title: String, body: String, optional: bool) -> void:
	title_label.text = title
	body_label.text = body
	body_label.visible = body != ""
	close_button.visible = optional
	if elements == null:
		return
	for element in elements:
		var scene: PackedScene = SCENES_BY_KIND.get(element.kind)
		if scene == null:
			push_warning("Unknown PromptElement kind: %d" % element.kind)
			continue
		var instance := scene.instantiate()
		instance.user_action.connect(_on_element_action)
		elements_container.add_child(instance)
		instance.set_element_data(element)

func _on_element_action(response: WharfkitPromptResponse) -> void:
	user_responded.emit(response)

func _on_close_pressed() -> void:
	user_responded.emit(WharfkitPromptResponse.closed())
