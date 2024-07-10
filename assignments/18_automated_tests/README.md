# Test

- `cargo test` compiles your code in test mode and runs the resulting test binary.

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4, "Result not equal to 4.");
        assert_ne!(result, 2, "Result equal to 2.");
    }

    #[test]
    #[should_panic] // Use should_panic
    fn it_dont_work() {
        panic!("This function will trigger warning");
    }

    #[test] // Use Result type for test
    fn it_works_with_result() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
```

### Controlling Test Runs

- By default, multiple tests are run in parallel using threads. Therefore, ensure that they don't depends on the share state to avoid racing condition.
    - Alternatively, use `--test-threads=1` to avoid parallelism.
- Use `--show-output` flag to display test standard output in terminal.
- Running of tests can be specified as argument after `cargo test test_identifier` where all test with `test_identifier` will be run.
