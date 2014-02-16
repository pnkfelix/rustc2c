use std::c_str::CString;
use std::libc::c_char;
use std::ptr;
use std::vec;

#[no_mangle]
pub extern "C" fn run_compiler(args: **c_char, len: u32) -> u32 {
    use rustc::run_compiler;
    let mut some_failed = false;
    let mut strs = vec::with_capacity(len as uint);
    unsafe {
        ptr::array_each_with_len(args, len as uint, |arg| {
                let cstr = CString::new(arg, false);
                match cstr.as_str() {
                    Some(s) => strs.push(s.to_owned()),
                    None    => some_failed = true,
                }
            });
    }
    if some_failed {
        return 1;
    } else {
        println!("About to call rustc::run_compiler");
        run_compiler(strs);
        println!("Returned from rustc::run_compiler");
        return 0;
    }
}
