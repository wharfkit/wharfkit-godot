extends Node

func _ready() -> void:
	test_runtime_singleton_present()
	test_class_db_has_classes()
	test_error_factories()
	test_prompt_element_factories()
	test_prompt_response_factories()
	test_platform_name()
	print("All tests passed.")

func test_runtime_singleton_present() -> void:
	assert(Engine.has_singleton("WharfkitRuntime"), "WharfkitRuntime singleton not registered")
	var rt = Engine.get_singleton("WharfkitRuntime")
	assert(rt != null, "Engine.get_singleton returned null")
	assert(rt.ping(), "WharfkitRuntime.ping() returned false")

func test_class_db_has_classes() -> void:
	for cls in [
		"WharfkitWalletPlugin", "WharfkitUserInterface", "WharfkitDeepLinkBridge",
		"NoOpDeepLinkBridge", "WharfkitError",
		"WharfkitPromptArgs", "WharfkitPromptResponse", "WharfkitPromptElement",
		"WharfkitPromptHandle", "WharfkitRuntime",
		"WharfkitLoginContext", "WharfkitTransactContext",
	]:
		assert(ClassDB.class_exists(cls), "%s not in ClassDB" % cls)

func test_error_factories() -> void:
	var rejected = WharfkitError.user_rejected("nope")
	assert(rejected.message == "nope", "user_rejected message")
	assert(rejected.retryable == false, "user_rejected not retryable")

	var network = WharfkitError.network("offline")
	assert(network.retryable == true, "network retryable")

	var chain = WharfkitError.chain_rejected("dup tx", 3050003)
	assert(chain.code == 3050003, "chain_rejected code")

	var cancelled = WharfkitError.cancelled("stop")
	assert(cancelled.retryable == true, "cancelled retryable")

func test_prompt_element_factories() -> void:
	var qr = WharfkitPromptElement.qr("esr://foo")
	assert(qr.data == "esr://foo", "qr data")

	var link = WharfkitPromptElement.link("launch", "esr://bar", "Open Anchor")
	assert(link.id == "launch", "link id")
	assert(link.href == "esr://bar", "link href")
	assert(link.label == "Open Anchor", "link label")

	var btn = WharfkitPromptElement.button("manual", "Sign manually")
	assert(btn.id == "manual", "button id")

	var countdown = WharfkitPromptElement.countdown("expires", "Expires in", 12345)
	assert(countdown.end_unix_ms == 12345, "countdown end_unix_ms")

	var accept = WharfkitPromptElement.accept("OK")
	assert(accept.label == "OK", "accept label")

	var close = WharfkitPromptElement.close()
	assert(close != null, "close element constructible")

func test_prompt_response_factories() -> void:
	var accepted = WharfkitPromptResponse.accepted()
	assert(accepted != null, "accepted constructible")

	var button = WharfkitPromptResponse.button_pressed("manual")
	assert(button.id == "manual", "button_pressed id")

	var link = WharfkitPromptResponse.link_opened("launch")
	assert(link.id == "launch", "link_opened id")

func test_platform_name() -> void:
	var name = OS.get_name()
	assert(name in ["macOS", "Windows", "Linux", "iOS", "Android", "Web"], "Platform name recognised: %s" % name)
