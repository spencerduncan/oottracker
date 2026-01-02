/// Compile-time tests for procedural macros using trybuild.
///
/// These tests verify that:
/// - Valid macro invocations compile successfully
/// - Invalid macro invocations produce expected compile errors

#[test]
fn ui() {
    let t = trybuild::TestCases::new();

    // flags_list! macro tests - passing tests
    t.pass("tests/ui/flags_list_valid.rs");
    t.pass("tests/ui/flags_list_private_struct.rs");

    // flags_list! macro tests - failing tests
    t.compile_fail("tests/ui/flags_list_invalid.rs");
    t.compile_fail("tests/ui/flags_list_out_of_bounds_index.rs");
    t.compile_fail("tests/ui/flags_list_invalid_index.rs");
    t.compile_fail("tests/ui/flags_list_missing_value.rs");
    t.compile_fail("tests/ui/flags_list_invalid_flag_name.rs");

    // scene_flags! macro tests - passing tests
    t.pass("tests/ui/scene_flags_valid.rs");
    t.pass("tests/ui/scene_flags_private_struct.rs");

    // scene_flags! macro tests - failing tests
    t.compile_fail("tests/ui/scene_flags_invalid.rs");
    t.compile_fail("tests/ui/scene_flags_missing_name.rs");
    t.compile_fail("tests/ui/scene_flags_invalid_index.rs");
    t.compile_fail("tests/ui/scene_flags_invalid_flag_value.rs");
    t.compile_fail("tests/ui/scene_flags_missing_flag_value.rs");
}
