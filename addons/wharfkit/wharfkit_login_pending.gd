class_name WharfkitLoginPending
extends WharfkitPending

signal login_done(session: WharfkitSession)
signal login_failed(error: WharfkitError)

func _dispatch_done(session) -> void:
	login_done.emit(session)

func _dispatch_failed(error) -> void:
	login_failed.emit(error)
