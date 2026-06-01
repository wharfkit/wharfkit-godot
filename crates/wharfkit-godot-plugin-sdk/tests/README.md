# SDK Test Harness Notes

The `dict`, `signal`, `login_context_view`, and `transact_context_view`
fixtures exercise Godot-owned values (`Dictionary`, `Array`, `Variant`,
`Gd<Object>`). With `godot`/`gdext` 0.3.5 these values require an initialized
Godot engine; running them as plain Cargo integration tests can segfault before
Rust can report a failure.

Those fixtures are therefore `#[ignore]` until this workspace adds a real
gdext integration-test runner. The live contract path is covered by the
headless mocked smoke in `example/tests/test_session_kit.gd`, which asserts the
GDScript orchestrator provides the login context shape and the transact
`ctx.resolved_request` dictionary that Rust plugin cdylibs consume through the
SDK view wrappers.
