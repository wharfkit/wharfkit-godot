extends Node

func _ready() -> void:
	test_jungle4_chain_id()
	test_name_construction()
	test_name_equality()
	print("All tests passed.")

func test_jungle4_chain_id() -> void:
	var chain = WharfkitChains.jungle4()
	var id_hex = chain.chain_id_hex
	assert(id_hex == "73e4385a2708e6d7048834fbc1079f2fabb17b3c125b146af438971e90716c4d", "Jungle4 chain id mismatch: got %s" % id_hex)

func test_name_construction() -> void:
	var n = WharfkitName.from("alice")
	assert(n.to_string_value() == "alice", "Name string roundtrip")

func test_name_equality() -> void:
	var a = WharfkitName.from("teamgreymass")
	var b = WharfkitName.from("teamgreymass")
	assert(a.equals(b), "Equal names should be equal")
