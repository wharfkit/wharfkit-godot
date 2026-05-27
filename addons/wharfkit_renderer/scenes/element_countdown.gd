extends Control

signal user_action(response: WharfkitPromptResponse)

var _end_unix_ms: int = 0
var _custom_label: String = ""
var _expired_emitted: bool = false
var _last_shown_s: int = -1
var _label: Label

func set_element_data(element) -> void:
	_end_unix_ms = int(element.end_unix_ms)
	_custom_label = String(element.label)
	if _label != null:
		_update_label(_remaining_ms())

func _ready() -> void:
	_label = $Label
	_update_label(_remaining_ms())

func _process(_delta: float) -> void:
	if _expired_emitted:
		return
	var remaining_ms := _remaining_ms()
	if remaining_ms <= 0:
		_expired_emitted = true
		if _label != null:
			_label.text = "%s — expired" % _custom_label
		user_action.emit(WharfkitPromptResponse.expired())
		set_process(false)
		return
	_update_label(remaining_ms)

func _remaining_ms() -> int:
	var now_ms := int(Time.get_unix_time_from_system() * 1000.0)
	return _end_unix_ms - now_ms

func _update_label(remaining_ms: int) -> void:
	if _label == null:
		return
	var remaining_s := int(remaining_ms / 1000)
	if remaining_s < 0:
		remaining_s = 0
	if remaining_s == _last_shown_s:
		return
	_last_shown_s = remaining_s
	_label.text = "%s — %ds" % [_custom_label, remaining_s]
