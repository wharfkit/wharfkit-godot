extends TestRunner

func _ready() -> void:
	test_anchor_plugin_class_registered()
	test_anchor_plugin_instantiable()
	test_anchor_plugin_methods_present()
	test_anchor_plugin_signals_present()
	test_anchor_plugin_ping()
	test_anchor_plugin_id_and_buoy_url()
	test_anchor_plugin_buoy_relay_override()
	test_eosio_token_class_registered()
	test_eosio_token_transfer_action_shape()
	if not _failed:
		print("All tests passed.")

func test_anchor_plugin_class_registered() -> void:
	_check(ClassDB.class_exists("WharfkitWalletPluginAnchor"),
		"WharfkitWalletPluginAnchor not in ClassDB")

func test_anchor_plugin_instantiable() -> void:
	var anchor := WharfkitWalletPluginAnchor.new()
	_check(anchor != null, "WharfkitWalletPluginAnchor.new() returned null")

func test_anchor_plugin_methods_present() -> void:
	var anchor := WharfkitWalletPluginAnchor.new()
	for m in ["_login", "_sign", "_logout"]:
		_check(anchor.has_method(m),
			"WharfkitWalletPluginAnchor missing method %s" % m)

func test_anchor_plugin_signals_present() -> void:
	var anchor := WharfkitWalletPluginAnchor.new()
	for s in ["login_done", "login_failed", "sign_done", "sign_failed",
			"logout_done", "logout_failed"]:
		_check(anchor.has_signal(s),
			"WharfkitWalletPluginAnchor missing signal %s" % s)

func test_anchor_plugin_ping() -> void:
	var anchor := WharfkitWalletPluginAnchor.new()
	_check(anchor.ping(), "WharfkitWalletPluginAnchor.ping() returned false")

func test_anchor_plugin_id_and_buoy_url() -> void:
	var anchor := WharfkitWalletPluginAnchor.new()
	_check(String(anchor.get_id()) == "anchor",
		"WharfkitWalletPluginAnchor.get_id() = %s (expected 'anchor')"
		% String(anchor.get_id()))
	_check(String(anchor.buoy_url()) == "https://cb.anchor.link",
		"WharfkitWalletPluginAnchor.buoy_url() default = %s"
		% String(anchor.buoy_url()))

func test_anchor_plugin_buoy_relay_override() -> void:
	var anchor := WharfkitWalletPluginAnchor.new()
	anchor.set_buoy_relay("http://127.0.0.1:8080")
	_check(String(anchor.buoy_url()) == "http://127.0.0.1:8080",
		"WharfkitWalletPluginAnchor.set_buoy_relay() not picked up: %s"
		% String(anchor.buoy_url()))

func test_eosio_token_class_registered() -> void:
	_check(ClassDB.class_exists("WharfkitEosioToken"),
		"WharfkitEosioToken not in ClassDB")

func test_eosio_token_transfer_action_shape() -> void:
	var token := WharfkitEosioToken.new()
	var action := token.transfer("alice", "bob", "1.0000 EOS", "thanks", "alice", "active")
	_check(action is Dictionary,
		"transfer() must return a Dictionary, got %s" % typeof(action))
	_check(String(action["account"]) == "eosio.token",
		"action.account = %s" % String(action.get("account", "")))
	_check(String(action["name"]) == "transfer",
		"action.name = %s" % String(action.get("name", "")))
	var auth = action.get("authorization", [])
	_check(auth is Array and auth.size() == 1,
		"action.authorization must be a 1-element Array")
	if auth is Array and auth.size() == 1:
		var first = auth[0]
		_check(String(first["actor"]) == "alice",
			"authorization[0].actor = %s" % String(first.get("actor", "")))
		_check(String(first["permission"]) == "active",
			"authorization[0].permission = %s" % String(first.get("permission", "")))
	var data = action.get("data", {})
	_check(data is Dictionary, "action.data must be a Dictionary")
	if data is Dictionary:
		_check(String(data.get("from", "")) == "alice", "data.from")
		_check(String(data.get("to", "")) == "bob", "data.to")
		_check(String(data.get("quantity", "")) == "1.0000 EOS", "data.quantity")
		_check(String(data.get("memo", "")) == "thanks", "data.memo")
