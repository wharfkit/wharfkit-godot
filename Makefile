.PHONY: help build build-all check fmt-check clippy test stage release run editor smoke clean

.DEFAULT_GOAL := help

# Load .env if present; export so child shells (cargo, scripts) see the vars.
-include .env
export

TARGET_DARWIN := aarch64-apple-darwin
TARGET_IOS    := aarch64-apple-ios
TARGET_IOSSIM := aarch64-apple-ios-sim

GODOT_BIN ?= /Applications/Godot.app/Contents/MacOS/Godot
EXAMPLE_DIR := example

help:
	@echo "wharfkit-godot — make targets:"
	@echo ""
	@echo "  build       Build the core cdylib for macOS arm64 (default dev target)."
	@echo "  build-all   Build the core cdylib for macOS arm64, iOS device, iOS sim."
	@echo "  check       Run fmt-check + clippy + test."
	@echo "  test        Run cargo tests in release mode."
	@echo "  stage       Build + populate example/addons/ with binaries and source mirror."
	@echo "  run         Launch the example app (runs main.tscn, no editor)."
	@echo "  editor      Open the example project in the Godot editor."
	@echo "  smoke       Headless run of the mocked smoke test (no network)."
	@echo "  release     Build all targets + populate canonical addons/*/lib/ for distribution."
	@echo "  clean       Remove target/ and the example/addons/ mirror."

build:
	cargo build --release --target $(TARGET_DARWIN)

build-all:
	cargo build --release --target $(TARGET_DARWIN)
	cargo build --release --target $(TARGET_IOS)
	cargo build --release --target $(TARGET_IOSSIM)

check: fmt-check clippy test

fmt-check:
	cargo fmt --check

clippy:
	cargo clippy --release --target $(TARGET_DARWIN) --all-targets -- -D warnings

test:
	cargo test --release --target $(TARGET_DARWIN)

stage: build
	./scripts/stage.sh

release: build-all
	./scripts/release.sh

run: stage
	$(GODOT_BIN) --path $(EXAMPLE_DIR)

editor: stage
	$(GODOT_BIN) --editor --path $(EXAMPLE_DIR)

smoke: stage
	$(GODOT_BIN) --headless --path $(EXAMPLE_DIR) res://tests/test_session_kit.tscn --quit-after 60

clean:
	cargo clean
	rm -rf $(EXAMPLE_DIR)/addons
