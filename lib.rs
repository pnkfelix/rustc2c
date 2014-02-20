// rustc2c exposes various bindings from Rust's libsyntax and librustc
// crates as unmanagled symbols callable from C (or other languages
// that can bind to C libraries).

// For now, leave this up to the user (see #11573 for why).
// #[crate_type="dylib"];

#[crate_id="rustc2c"];

extern mod extra;
extern mod native;
extern mod green;
extern mod syntax;
extern mod rustc;

pub use smoke_test::{callee, caller, caller_2nd, hello, mul3, add5, twice};

pub use compile::{run_compiler};

mod smoke_test;
mod read;
mod compile;
mod print;

#[no_mangle]
pub extern "C" fn call_with_native_runtime(
    argc: u32,
    argv: **u8,
    ctxt: *u8,
    f: extern "C" fn(ctxt: *u8))
    -> i32 {
    let code = native::start(argc as int, argv, proc() {
            f(ctxt)
        });
    return code as i32;
}
