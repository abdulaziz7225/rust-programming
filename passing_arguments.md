# Passing Arguments to a Program with Cargo

When running a program with `cargo run`, **flags after the command are interpreted by Cargo itself** unless you explicitly separate them with `--`. Use `--` to tell Cargo : "Stop parsing flags for yourself; everything after belongs to the program."

```bash
$ cargo run -- -n Hello world
Args { inner: ["target/debug/<PROGRAM_NAME>", "-n", "Hello", "world"] }
```
