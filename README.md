# VaultAPI-Client

[![made-with-rust][rust-logo]][rust-src-page]

[![crates.io][crates-logo]][crate]

[![build][gh-logo]][build]
[![none-shall-pass][nsp-logo]][nsp]

Client application for [VaultAPI] server

### Environment Variables
Env vars can either be loaded from any plaintext files.

- **ENV_FILE** - Plaintext file to read the env vars. Defaults to `.env`
- **VAULT_ADDRESS** - VaultAPI server URL. Defaults to http://0.0.0.0:8080
- **APIKEY** - API key to authenticate the VaultAPI server.
- **TRANSMIT_KEY_LENGTH** - AES key length for transit encryption. Defaults to `32`
- **TRANSIT_TIME_BUCKET** - Interval for which the transit epoch should remain constant. Defaults to `60`

### Commandline Arguments

- **--env_file** - Plaintext file to read the env vars. Defaults to `.env`
- **--cipher** - Cipher text to decrypt the secret to a JSON value.
- **--table** - Name of the table to retrieve the secret from.
- **--get-secret** - Get the value of a particular secret key.
- **--get-secrets** - Get the values of multiple keys using a comma separated list.
- **--get-table** - Get all the secrets stored in a table.

## Crate
[https://crates.io/crates/VaultAPI-Client][crate]

### Cargo Docs - Official Runbook
[https://docs.rs/VaultAPI-Client/latest/][docs]

**Generator**
```shell
cargo doc --document-private-items --no-deps
```

## Linting
### Requirement
```shell
rustup component add clippy
```
### Usage
```shell
cargo clippy --no-deps --fix
```

## License & copyright

&copy; Vignesh Rao

Licensed under the [MIT License][license]

[rust-src-page]: https://www.rust-lang.org/
[rust-logo]: https://img.shields.io/badge/Made%20with-Rust-black?style=for-the-badge&logo=Rust
[license]: https://github.com/thevickypedia/VaultAPI-Client/blob/main/LICENSE
[VaultAPI]: https://github.com/thevickypedia/VaultAPI
[docs]: https://docs.rs/VaultAPI-Client/latest/
[nsp]: https://github.com/thevickypedia/VaultAPI-Client/actions/workflows/none.yml
[crate]: https://crates.io/crates/VaultAPI-Client
[crates-logo]: https://img.shields.io/crates/v/VaultAPI-Client.svg
[build]: https://github.com/thevickypedia/VaultAPI-Client/actions/workflows/rust.yml
[gh-logo]: https://github.com/thevickypedia/VaultAPI-Client/actions/workflows/rust.yml/badge.svg
[nsp-logo]: https://github.com/thevickypedia/VaultAPI-Client/actions/workflows/none.yml/badge.svg
