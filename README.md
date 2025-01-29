# c-lib-macros
This library contains macros for raw C FFI bindings. This will reduce boilerplate code allowing developers to more easily write Rust code that interfaces with C. The C interface can act as an intermediate language between Rust and other programming languages, allowing Rust functions to be called from other languages. This will therefore allow Rust libraries to be more easily exposed to other languages.

# Contents
- `c-lib-macros` is the library for the macros
- `tests` contains integration tests
- `scripts` contains scripts for running integration tests
- `logs` contains logs for integration tests

# Integration Tests

To run an integration test, you can use the following command:

```bash
sh scripts/run_integration_tests.sh > logs/integration_test.log
```

Currently, the tests demonstrate within `tests/test-runner/src/lib.rs` demonstrate:
- Dynamic loading of the compiled `test-lib-wrapper` library
- Calling simple C-compatible functions with different parameter types:
  - Basic integer operations (`add` function)
  - String handling and printing (`print_two_statements` function)
- Proper type conversion between Rust and C for integers and strings
- Error handling for null pointers and invalid string data

# TODO
- [x] make test block for test runner
- [ ] build C structs for String, EmptyReturn, and some other inputs and outputs (c-lib-utils)
- [ ] build C functions for freeing up the memory of structs (c-lib-utils)
- [ ] build Rust macros for handling inputs for structs (c-lib-utils)