use libloading::{Library, Symbol};
use std::os::raw::c_int;
use std::os::raw::c_char;
use std::ffi::CString;


#[cfg(test)]
mod tests {
    use super::*;

    // helper function to load the dynamic library
    fn setup() -> Library {
        let lib_path = "../assets/libtest_lib_wrapper.dylib"; // Use `.dll` on Windows, `.dylib` on macOS, `.so` on Linux
        unsafe { Library::new(lib_path).expect("Failed to load library") }
    }

    #[test]
    fn test_add() {
        let lib = setup();
        unsafe {
            let add: Symbol<unsafe extern "C" fn(i32, i32) -> c_int> =
                lib.get(b"add").expect("Failed to load function `add`");

            let result = add(3, 5);
            assert_eq!(result, 8);
        }
    }

    #[test]
    fn test_print_two_statements() {
        let lib = setup();
        unsafe {
            let print_two_statements: Symbol<unsafe extern "C" fn(*mut c_char, *mut c_char) -> c_int> =
                lib.get(b"print_two_statements").expect("Failed to load function `print_two_statements`");
            
            let hello = std::ffi::CString::new("Hello").unwrap();
            let world = std::ffi::CString::new("World!").unwrap();

            let result = print_two_statements(hello.as_ptr() as *mut c_char, world.as_ptr() as *mut c_char);
            assert_eq!(result, 0);
        }    
    }
}
