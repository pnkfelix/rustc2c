// rustc2c exposes various bindings from Rust's libsyntax and librustc
// crates as unmanagled symbols callable from C (or other languages
// that can bind to C libraries).

// For now, leave this up to the user (see #11573 for why).
// #[crate_type="dylib"];

#[crate_id="rustc2c"];

extern mod extra;
extern mod syntax;

use std::libc::c_char;
use std::c_str::CString;

#[no_mangle]
pub extern "C" fn hello()
{
    println!("Hello world from Rust");
}

#[no_mangle]
pub extern "C" fn callee(from: *c_char)
{
    let from = unsafe { CString::new(from, false) };
    println!("called {:s} from {:s}", "Rust", from.as_str().unwrap_or("(nonutf8)"));
}

#[no_mangle]
pub extern "C" fn caller(f: extern "C" fn(from: *c_char))
{
    let from = "Rust";
    f(from.as_ptr() as *c_char);
}

#[no_mangle]
pub extern "C" fn caller_2nd(f: extern "C" fn(g: extern "C" fn(from: *c_char)))
{
    extern "C" fn g(from: *c_char) {
        let from = unsafe { CString::new(from, false) };
        println!("called Rust from {:s}, and the latter from Rust",
                 from.as_str().unwrap_or("(nonutf8)"));
    }
    f(g);
}


#[no_mangle]
pub extern "C" fn twice(f: extern "C" fn(arg: u32) -> u32, input: u32) -> u32
{
    f(f(input))
}

#[no_mangle]
pub extern "C" fn mul3(arg: u32) -> u32 { arg * 3 }

#[no_mangle]
pub extern "C" fn add5(arg: u32) -> u32 { arg + 5 }

#[test]
fn hi() {
    println!("Hello World");
}

#[cfg(test)]
mod bench {
    use extra::test::BenchHarness;

    #[bench]
    fn b(_bh: &mut BenchHarness) {
        let mut _x = 0;
        _bh.iter(|| { _x += 1; });
        _x += 1;
    }
}
