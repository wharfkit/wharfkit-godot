class_name EosioToken
extends RefCounted

const ACCOUNT := "eosio.token"

static func transfer(
	from: String,
	to: String,
	quantity: String,
	memo: String,
	authorizer: String = "",
	permission: String = "active",
) -> Dictionary:
	if authorizer == "":
		authorizer = from
	return {
		"account": ACCOUNT,
		"name": "transfer",
		"authorization": [{"actor": authorizer, "permission": permission}],
		"data": {
			"from": from,
			"to": to,
			"quantity": quantity,
			"memo": memo,
		},
	}
