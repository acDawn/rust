// FIXME: if/when the output of the test harness can be tested on its own, this test should be
// adapted to use that, and that normalize line can go away

// compile-flags:--test
// normalize-stdout-test: "src/test/rustdoc-ui" -> "$$DIR"
// failure-status: 101

/// ```compile_fail,E0004
/// let x: () = 5i32;
/// ```
pub struct Foo;
