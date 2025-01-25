use libloading::{Library, Symbol};
use std::os::raw::c_int;

// TODO => translate these basic tests to a cfg test block 

fn main() {
    println!("running the test-runner");
    let lib_path = "tests/assets/libtest_lib_wrapper.dylib"; // Use `.dll` on Windows, `.dylib` on macOS, `.so` on Linux

    // Load the library
    let lib = unsafe {Library::new(lib_path).expect("Failed to load library")};

    // Load the `add` function
    unsafe {
        let add: Symbol<unsafe extern "C" fn(c_int, c_int) -> c_int> =
            lib.get(b"add").expect("Failed to load function `add`");

        // Call the `add` function
        let result = add(3, 5);
        println!("testing the add function from the c library");
        assert_eq!(result, 8);
        println!("add function test passed");
    }

    // The library is automatically unloaded when it goes out of scope
}