# TODO

## Futures Port Maturity

- Add CI coverage for all supported backend configurations:
  - `cargo test`
  - `cargo test --no-default-features --features futures`
  - `cargo test --no-default-features --features futures,xattr`
  - `cargo check --all-features`, expected to fail with the mutual-exclusion error.
- Run the CI matrix on Linux, macOS, and Windows.
- Pay particular attention to Windows behavior for symlinks, long paths, permissions, and timestamps.
- Add a downstream smoke test that consumes this crate with `default-features = false` and `features = ["futures"]`.

## Documentation

- Add backend-specific examples for create, list, and unpack workflows.
- Document public API differences from `astral-tokio-tar`.
- Make the futures `Builder::new` termination behavior prominent: futures users must call `finish()` or `into_inner()` to guarantee archive termination bytes.
- Review README wording before publishing under the `astral-futures-tar` name.

## Performance

- Treat the current ignored `Instant` tests as smoke benchmarks only.
- Add larger filesystem fixtures, ideally 256 MiB to 1 GiB.
- Report median and p95 in addition to best-of-N timings.
- Randomize or alternate backend run order when comparing Tokio and futures.
- Add a pure async `copy` microbenchmark outside tar parsing.
- Consider separate warm-cache and cold-ish filesystem benchmark modes.
