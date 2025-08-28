# Rust Test Execution Order

By default, **Rust tests are not guaranteed to run in the order they are declared**.  

This happens because Rust encourages **safe concurrent programming**. The test harness takes advantage of concurrency and runs multiple tests in parallel across different threads. As a result, test results may appear in a different order each time you run them.

✅ This is a **feature**, not a bug.

---

## Running Tests in a Fixed Order

If you need tests to run sequentially (e.g., for debugging or when tests depend on shared state), you can force Rust to run them on a **single thread**:

```bash
cargo test -- --test-threads=1
