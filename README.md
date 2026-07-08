# `astral-futures-tar`

An async tar archive reader and writer with selectable Tokio and futures backends.

## Features

Tokio remains the default backend:

```toml
astral-futures-tar = "0.6"
```

For no-Tokio builds, disable default features and enable the futures backend:

```toml
astral-futures-tar = { version = "0.6", default-features = false, features = ["futures"] }
```

The `tokio` and `futures` features are mutually exclusive. The futures backend uses
`async-fs` for filesystem helpers and `futures-lite` I/O traits and combinators.

When using the futures backend, call `finish()` or `into_inner()` on `Builder` to write
archive termination bytes. Tokio builds keep the historical best-effort drop-time
termination behavior, but futures builds do not block or spawn hidden executor work from
`Drop`.

## Performance Smoke Tests

Ignored `Instant`-based performance tests are available for backend comparisons:

```sh
cargo test --release perf -- --ignored --nocapture
cargo test --release --no-default-features --features futures perf -- --ignored --nocapture
```

## Provenance

This crate is a fork of [`edera-dev/tokio-tar`](https://github.com/edera-dev/tokio-tar),
which was a fork of [`vorot93/tokio-tar`](https://github.com/vorot93/tokio-tar),
which was a fork of [`dignifiedquire/async-tar`](https://github.com/dignifiedquire/async-tar),
which is based on [`alexcrichton/tar-rs`](https://github.com/alexcrichton/tar-rs).

As compared to the async tar crates, this crate includes a variety of performance improvements
and missing patches from [`alexcrichton/tar-rs`](https://github.com/alexcrichton/tar-rs).

As compared to [`alexcrichton/tar-rs`](https://github.com/alexcrichton/tar-rs), this crate features
the following modifications:

- Setting `preserve_permissions` to `false` will avoid setting _any_ permissions on extracted files.
  In [`alexcrichton/tar-rs`](https://github.com/alexcrichton/tar-rs), setting `preserve_permissions`
  to `false` will still set read, write, and execute permissions on extracted files, but will avoid
  setting extended permissions (e.g., `setuid`, `setgid`, and `sticky` bits).
- Setting `allow_external_symlinks` to `false` will avoid extracting symlinks that point outside the
  unpack target. Operations that _write_ outside the unpack directory are _always_ denied; but by
  default, symlinks that _read_ outside the unpack directory are allowed.

See the [changelog](CHANGELOG.md) for a more detailed list of changes.

## Security Considerations

Securely extracting an arbitrary (potentially malicious) archive to disk requires understanding the properties of the host OS and filesystem. Failing to account for how different OSes and filesystems process byte sequences in paths may allow an attacker to bypass application-level path filters by exploiting behaviors like:

- Unicode normalization: some OSes (like macOS) use Unicode's NFD normalization form for path handling, meaning that distinct byte sequences within an archive can normalize to the same path on disk.
- Case folding: Some filesystems are case-insensitive or case-preseving, meaning that entries whose paths only vary by case may result in the same path on disk. For example, both APFS (macOS) and NTFS (Windows) exhibit case-insensitive/preserving behavior.
- Path equivalence: Other OS- or filesystem-specific rules that treat distinct byte sequences as the same file.

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
