# c-lib-macros
A library for supplying macros for raw C interfaces. 


# Integration Tests

To run an integration test, you can use the following command:

```bash
sh scripts/run_integration_tests.sh > logs/integration_test.log
```


# TO DO List

- [ ] make test block for test runner
- [ ] build C structs for String, EmptyReturn, and some other inputs and outputs (c-lib-utils)
- [ ] build C functions for freeing up the memory of structs (c-lib-utils)
- [ ] build Rust macros for handling inputs for structs (c-lib-utils)
