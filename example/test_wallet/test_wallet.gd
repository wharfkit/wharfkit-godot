class_name TestGameWallet
extends WharfkitWalletPlugin

const SYNTHETIC_SIGNATURE := "SIG_K1_synthetic_signature_for_smoke_testing_purposes_only_do_not_use_in_production"

@export var actor: String = "alice"
@export var permission: String = "active"

var login_calls: int = 0
var sign_calls: int = 0
var logout_calls: int = 0
var last_login_ctx
var last_sign_request
var last_sign_ctx

func _login(ctx) -> void:
	login_calls += 1
	last_login_ctx = ctx
	var chain_id := ""
	if ctx is Dictionary:
		var chain = ctx.get("chain", {})
		if chain is Dictionary:
			chain_id = chain.get("id", "")
	var response := {
		"chain": chain_id,
		"permission_level": {
			"actor": actor,
			"permission": permission,
		},
	}
	login_done.emit(response)

func _sign(request, ctx) -> void:
	sign_calls += 1
	last_sign_request = request
	last_sign_ctx = ctx
	var actions: Array = []
	if request is Dictionary:
		actions = request.get("actions", [])
	var response := {
		"signatures": [SYNTHETIC_SIGNATURE],
		"action_count": actions.size(),
	}
	if request is Dictionary and request.get("broadcast", false):
		response["transaction_id"] = "mocked-transaction-id"
	sign_done.emit(response)

func _logout(_ctx) -> void:
	logout_calls += 1
	logout_done.emit()
