// rustc2c exposes various bindings from Rust's libsyntax and librustc
// crates as unmanagled symbols callable from C (or other languages
// that can bind to C libraries).

// For now, leave this up to the user (see #11573 for why).
// #[crate_type="dylib"];

#[crate_id="rustc2c"];

extern mod extra;
extern mod syntax;

pub use smoke_test::{callee, caller, caller_2nd, hello, mul3, add5};

mod smoke_test;
