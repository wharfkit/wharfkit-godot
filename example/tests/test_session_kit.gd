extends TestRunner

const TestUserInterface = preload("res://test_ui/test_ui.gd")
const TestGameWallet = preload("res://test_wallet/test_wallet.gd")

var _session: WharfkitSession

func _ready() -> void:
	await test_session_kit_class_registered()
	await test_login_flow()
	await test_transact_flow()
	await test_classes_present()
	if not _failed:
		print("All tests passed.")
		print("MOCKED SMOKE: PASS")

func test_classes_present() -> void:
	for c in ["WharfkitChains", "WharfkitChainDefinition", "WharfkitWalletPlugin",
			"WharfkitUserInterface", "WharfkitEosioToken", "WharfkitWalletPluginAnchor"]:
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
	_check(session is WharfkitSession, "session is not a WharfkitSession")

	_session = session
	kit.queue_free()
	await get_tree().process_frame

func test_transact_flow() -> void:
	_check(_session != null, "no session from login flow — skipping transact")
	if _session == null:
		return

	var token := WharfkitEosioToken.new()
	var action = token.transfer("alice", "bob", "0.0001 EOS", "mocked smoke", "alice", "active")
	_check(action is Dictionary, "token.transfer did not return a Dictionary")

	var pending = _session.transact({"actions": [action]}, {"broadcast": false})
	_check(pending != null, "transact() returned null pending")
	var result = await pending.transact_done
	_check(result != null, "transact_done emitted null result")
	if result != null:
		var sigs = result.get("signatures", [])
		_check(sigs is Array and sigs.size() == 1,
			"expected 1 signature, got %d" % (sigs.size() if sigs is Array else -1))
	_check((_session._plugin as TestGameWallet).sign_calls == 1,
		"wallet._sign was not called exactly once")
	_session.queue_free()
	await get_tree().process_frame
