# Authoritah Certificate Authority Library

## Dependencies
- [Rust] toolchain
    - via Homebrew:
        ```sh
        brew install rustup-init && rustup-init --default-toolchain stable
        ```
- [sqlite3] library
    - via Homebrew:
        ```sh
        brew install sqlite
        ```

## Building
_Development Build & Test Process_
```sh
git clone https://github.com/UnnecessaryEngineering/authoritah.git
cd authoritah
cargo test --lib authoritah-ca
```

_Release Build Process_

The Authoritah CA library is designed for static linking inside a CA daemon implementation.
See [Authoritah CA daemon build process].

[Rust]: https://rust.rs
[sqlite3]: https://sqlite.org
[Authoritah CA daemon build process]: ../README.md#daemon-build