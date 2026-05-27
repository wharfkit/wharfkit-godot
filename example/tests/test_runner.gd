class_name TestRunner
extends Node

var _failed: bool = false

func _fail(msg: String) -> void:
	_failed = true
	push_error(msg)
	print("FAIL: %s" % msg)

func _check(cond: bool, msg: String) -> void:
	if not cond:
		_fail(msg)
