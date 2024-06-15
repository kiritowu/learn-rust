# Hello Cargo

- Cargo is package management tool in Rust.
- Use `cargo new <project-name>` to create a new Rust project using Cargo.
    - Inside `./<project-name>`, two new file will be created:
        - `Cargo.toml`: Describe package version and dependencies.
        - `src/main.rs`: Entrypoint for program.
- Use `cargo build` to build the project with executable in `./target/debug/<project-name>`.
- Use `cargo run` to build and run the project immediately.
- Use `cargo check` to build a project without an executable.
