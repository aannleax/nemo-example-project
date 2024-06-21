# Using Nemo in a Rust Project

The basic setup is quite simple. First create a new project:
```
cargo new project
```

Then, add nemo as a dependency in `Cargo.toml`

```
[dependencies]
nemo = { git = "https://github.com/knowsys/nemo" }
```

Then simply compile with `cargo build -r`

To see the output of this example:
```
cargo run -- datalog/program.rls
```
