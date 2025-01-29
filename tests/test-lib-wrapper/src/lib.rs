use std::os::raw::{c_char, c_int};
use std::ffi::CStr;


// when you compile a binary, the compiler moves stuff around and renames things to optimize it (called mangling)
// no mangle tells the compiler not to do that to preserve the original name of the function, so when the code is compiled, the C code can still call the function. C is unable to to find mangled function names.
// more info: https://medium.com/@comsamtom/the-mangle-in-rust-language-512ee113dd73
#[no_mangle]
pub extern "C" fn add(a: c_int, b: c_int) -> c_int {
    a + b
}


// 0 = success 1 = failure
#[no_mangle]
pub extern "C" fn print_two_statements(a: *mut c_char, b: *mut c_char) -> c_int {
    let name_str = match a.is_null() {
        true => return 1,
        false => match unsafe { CStr::from_ptr(a) }.to_str() {
            Ok(s) => s.to_string(),
            Err(_) => return 1,
        },
    };

    let age_str = match b.is_null() {
        true => return 1,
        false => match unsafe { CStr::from_ptr(b) }.to_str() {
            Ok(s) => s.to_string(),
            Err(_) => return 1,
        },
    };
    println!("First statement: {:?}", name_str);
    println!("Second statement: {:?}", age_str);
    0
}
