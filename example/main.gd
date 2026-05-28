extends Control

const WharfkitRenderer = preload("res://addons/wharfkit_renderer/wharfkit_renderer.gd")

const REFRESH_INTERVAL_SECS := 5.0
const TRANSFER_AMOUNT := "0.0001 EOS"
const TRANSFER_RECIPIENT := "teamgreymass"
const EXPLORER_BASE := "https://jungle4.unicove.com/transaction/"

@onready var balance_label: Label = $VBox/BalanceLabel
@onready var refresh_button: Button = $VBox/RefreshButton
@onready var login_button: Button = $VBox/LoginButton
@onready var transfer_button: Button = $VBox/TransferButton
@onready var status_label: Label = $VBox/StatusLabel
@onready var explorer_link: LinkButton = $VBox/ExplorerLink

var _chain: WharfkitChainDefinition
var _contract_kit: WharfkitContractKit
var _token_contract: WharfkitContract
var _accounts_name: WharfkitName
var _kit: WharfkitSessionKit
var _session: WharfkitSession
var _balance_timer: Timer
var _watched_actor: String = ""

func _ready() -> void:
	refresh_button.pressed.connect(_on_refresh_pressed)
	login_button.pressed.connect(_on_login_pressed)
	transfer_button.pressed.connect(_on_transfer_pressed)
	explorer_link.pressed.connect(_on_explorer_pressed)
	transfer_button.disabled = true
	explorer_link.visible = false

	_chain = WharfkitChains.jungle4()
	_contract_kit = WharfkitContractKit.for_chain(_chain)
	_accounts_name = WharfkitName.from("accounts")

	_balance_timer = Timer.new()
	_balance_timer.wait_time = REFRESH_INTERVAL_SECS
	_balance_timer.autostart = false
	_balance_timer.one_shot = false
	_balance_timer.timeout.connect(_on_balance_timer)
	add_child(_balance_timer)

	_watched_actor = TRANSFER_RECIPIENT
	_fetch_balance()
	_balance_timer.start()

func _on_refresh_pressed() -> void:
	_fetch_balance()

func _on_balance_timer() -> void:
	_fetch_balance()

func _fetch_balance() -> void:
	var actor := _watched_actor
	if actor == "" or actor == "?":
		balance_label.text = "(no actor)"
		return
	if _token_contract == null:
		_token_contract = _contract_kit.load(WharfkitName.from("eosio.token"))
		if _token_contract == null:
			balance_label.text = "%s: (contract load failed)" % actor
			return
	var table: WharfkitTable = _token_contract.table(_accounts_name, WharfkitName.from(actor))
	if table == null:
		balance_label.text = "%s: (table not available)" % actor
		return
	var balance: WharfkitAsset = table.first_token_balance()
	if balance == null:
		balance_label.text = "%s: (no balance row)" % actor
		return
	balance_label.text = "%s balance: %s" % [actor, balance.to_string_value()]
	print("%s balance: %s" % [actor, balance.to_string_value()])

func _on_login_pressed() -> void:
	status_label.text = "Connect with Anchor on your phone..."
	explorer_link.visible = false

	if _kit != null:
		_kit.queue_free()

	var ui := WharfkitRenderer.new()
	var wallet := WharfkitWalletPluginAnchor.new()

	_kit = WharfkitSessionKit.new()
	add_child(_kit)
	_kit.configure({
		"app_name": "WharfKit Godot Sample",
		"chains": [_chain],
		"ui": ui,
		"wallet_plugins": [wallet],
		"return_path": "wharfkit://",
	})

	var pending: WharfkitLoginPending = _kit.login()
	var session: WharfkitSession = await _await_pending(pending, "login_done", "login_failed")
	if session == null:
		return
	_session = session
	var actor: String = String(session.permission_level.get("actor", "?"))
	var permission: String = String(session.permission_level.get("permission", "?"))
	status_label.text = "Logged in as %s@%s" % [actor, permission]
	transfer_button.disabled = false
	print("Logged in as: %s@%s" % [actor, permission])

	_watched_actor = actor
	_fetch_balance()

func _on_transfer_pressed() -> void:
	if _session == null:
		status_label.text = "Not logged in."
		return
	status_label.text = "Signing transfer..."
	explorer_link.visible = false

	var actor: String = String(_session.permission_level.get("actor", ""))
	var permission: String = String(_session.permission_level.get("permission", "active"))
	var action := EosioToken.transfer(
		actor,
		TRANSFER_RECIPIENT,
		TRANSFER_AMOUNT,
		"WharfKit Godot sample transfer",
		actor,
		permission,
	)

	var pending: WharfkitTransactPending = _session.transact({"actions": [action]}, {"broadcast": true})
	var result = await _await_pending(pending, "transact_done", "transact_failed")
	if result == null:
		return
	var sigs: Array = result.get("signatures", [])
	var tx_id := String(result.get("transaction_id", ""))
	if tx_id != "":
		status_label.text = "Broadcast: %s..." % tx_id.substr(0, 16)
		explorer_link.text = "View transaction (%s...)" % tx_id.substr(0, 8)
		explorer_link.set_meta("tx_id", tx_id)
		explorer_link.visible = true
		print("Broadcast transaction: %s" % tx_id)
	else:
		status_label.text = "Signed (%d sig) but no transaction id" % sigs.size()
		print("Signed transfer — %d signatures, no broadcast id" % sigs.size())

	_fetch_balance()

func _on_explorer_pressed() -> void:
	var tx_id := String(explorer_link.get_meta("tx_id", ""))
	if tx_id == "":
		return
	OS.shell_open(EXPLORER_BASE + tx_id)

func _await_pending(pending: Object, done_signal: String, failed_signal: String) -> Variant:
	var resolution := {"settled": false, "payload": null}
	pending.connect(done_signal, func(p):
		if not resolution["settled"]:
			resolution["payload"] = p
			resolution["settled"] = true
	, CONNECT_ONE_SHOT)
	pending.connect(failed_signal, func(err):
		if not resolution["settled"]:
			var msg := "(no error)"
			if err != null and "message" in err:
				msg = String(err.message)
			status_label.text = "%s: %s" % [failed_signal, msg]
			resolution["settled"] = true
	, CONNECT_ONE_SHOT)
	while not resolution["settled"]:
		await get_tree().process_frame
	return resolution["payload"]
