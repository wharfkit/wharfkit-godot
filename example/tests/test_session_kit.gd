extends TestRunner

const TestUserInterface = preload("res://test_ui/test_ui.gd")
const TestGameWallet = preload("res://test_wallet/test_wallet.gd")

var _session: WharfkitSession

func _ready() -> void:
	await test_session_kit_class_registered()
	await test_login_flow()
	await test_transact_flow()
	await test_classes_present()
	if _failed:
		print("MOCKED SMOKE: FAIL")
		get_tree().quit(1)
	else:
		print("All tests passed.")
		print("MOCKED SMOKE: PASS")
		get_tree().quit(0)

func test_classes_present() -> void:
	for c in ["WharfkitChains", "WharfkitChainDefinition", "WharfkitWalletPlugin",
			"WharfkitUserInterface", "WharfkitWalletPluginAnchor"]:
		_check(ClassDB.class_exists(c), "ClassDB missing %s" % c)

func test_session_kit_class_registered() -> void:
	var kit = WharfkitSessionKit.new()
	_check(kit != null, "WharfkitSessionKit.new() returned null")
	if kit != null:
		kit.queue_free()
	await get_tree().process_frame

func test_login_flow() -> void:
	var kit := WharfkitSessionKit.new()
	add_child(kit)
	var ui := TestUserInterface.new()
	var wallet := TestGameWallet.new()
	kit.configure({
		"app_name": "Mocked smoke",
		"chains": [WharfkitChains.jungle4()],
		"return_path": "game://return",
		"ui": ui,
		"wallet_plugins": [wallet],
	})

	var pending = kit.login()
	_check(pending != null, "login() returned null pending")
	var session = await pending.login_done
	_check(session != null, "login_done emitted null session")
	if session != null:
		_check(session.permission_level.get("actor", "") == "alice",
			"session.permission_level.actor != alice")
		_check(session.permission_level.get("permission", "") == "active",
			"session.permission_level.permission != active")
	_check(wallet.login_calls == 1, "wallet._login was not called exactly once")
	_check(wallet.last_login_ctx is Dictionary,
		"wallet._login ctx was not a Dictionary")
	if wallet.last_login_ctx is Dictionary:
		var login_chain = wallet.last_login_ctx.get("chain")
		_check(login_chain is Dictionary,
			"login ctx.chain was not a Dictionary")
		if login_chain is Dictionary:
			_check(login_chain.get("id", "") != "",
				"login ctx.chain.id was empty")
			_check(login_chain.get("url", "") != "",
				"login ctx.chain.url was empty")
		_check(wallet.last_login_ctx.get("app_name", "") == "Mocked smoke",
			"login ctx.app_name did not match config")
		_check(wallet.last_login_ctx.get("return_path", "") == "game://return",
			"login ctx.return_path did not match config")
		_check(wallet.last_login_ctx.get("ui") == ui,
			"login ctx.ui did not match configured UI")
	_check(session is WharfkitSession, "session is not a WharfkitSession")

	_session = session
	kit.queue_free()
	await get_tree().process_frame

func test_transact_flow() -> void:
	_check(_session != null, "no session from login flow — skipping transact")
	if _session == null:
		return

	var action := EosioToken.transfer("alice", "bob", "0.0001 EOS", "mocked smoke", "alice", "active")
	_check(action is Dictionary, "EosioToken.transfer did not return a Dictionary")

	var pending = _session.transact({"actions": [action]}, {"broadcast": true})
	_check(pending != null, "transact() returned null pending")
	var result = await pending.transact_done
	_check(result != null, "transact_done emitted null result")
	if result != null:
		var sigs = result.get("signatures", [])
		_check(sigs is Array and sigs.size() == 1,
			"expected 1 signature, got %d" % (sigs.size() if sigs is Array else -1))
		_check(result.get("transaction_id", "") == "mocked-transaction-id",
			"transaction_id was not propagated from wallet response")
	var wallet := _session._plugin as TestGameWallet
	_check(wallet.sign_calls == 1,
		"wallet._sign was not called exactly once")
	_check(wallet.last_sign_ctx is Dictionary,
		"wallet._sign ctx was not a Dictionary")
	if wallet.last_sign_ctx is Dictionary:
		var resolved_request = wallet.last_sign_ctx.get("resolved_request")
		_check(resolved_request is Dictionary,
			"ctx.resolved_request was not a Dictionary")
		_check(resolved_request == wallet.last_sign_request,
			"ctx.resolved_request did not match the _sign request")
		var transact_chain = wallet.last_sign_ctx.get("chain")
		_check(transact_chain != null and transact_chain.has_method("url_value"),
			"transact ctx.chain did not expose url_value")
		if transact_chain != null and transact_chain.has_method("url_value"):
			_check(transact_chain.url_value() != "",
				"transact ctx.chain.url_value was empty")
	_session.queue_free()
	await get_tree().process_frame
