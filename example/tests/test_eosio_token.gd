extends TestRunner

func _ready() -> void:
	test_transfer_action_shape()
	test_transfer_authorizer_defaults_to_from()
	if not _failed:
		print("All tests passed.")

func test_transfer_action_shape() -> void:
	var action := EosioToken.transfer("alice", "bob", "1.0000 EOS", "thanks", "alice", "active")
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

func test_transfer_authorizer_defaults_to_from() -> void:
	var action := EosioToken.transfer("alice", "bob", "1.0000 EOS", "")
	var auth = action.get("authorization", [])
	if auth is Array and auth.size() == 1:
		_check(String(auth[0]["actor"]) == "alice",
			"default authorizer should be `from`")
		_check(String(auth[0]["permission"]) == "active",
			"default permission should be `active`")
