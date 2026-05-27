class_name WharfkitTransactPending
extends WharfkitPending

signal transact_done(result: Dictionary)
signal transact_failed(error: WharfkitError)

func _dispatch_done(result) -> void:
	transact_done.emit(result)

func _dispatch_failed(error) -> void:
	transact_failed.emit(error)
