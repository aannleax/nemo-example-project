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