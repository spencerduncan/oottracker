/// Compile-time tests for procedural macros using trybuild.
///
/// These tests verify that:
/// - Valid macro invocations compile successfully
/// - Invalid macro invocations produce expected compile errors

#[test]
fn ui() {
    let t = trybuild::TestCases::new();

    // flags_list! macro tests
    t.pass("tests/ui/flags_list_valid.rs");
    t.compile_fail("tests/ui/flags_list_invalid.rs");

    // scene_flags! macro tests
    t.pass("tests/ui/scene_flags_valid.rs");
    t.compile_fail("tests/ui/scene_flags_invalid.rs");
}
