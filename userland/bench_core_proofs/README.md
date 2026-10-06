# bench_core_proofs

Host tests for the benchmark summary arithmetic in `nonos-bench`: `summary.rs`,
`sort.rs`, `overhead.rs` and `tsc.rs`, mounted by `#[path]` and driven with runs
whose percentiles are known by hand. The cycle counter itself is not tested; on
the host `read_serialised` returns a constant. 12 tests.

Run with `cargo test --release` from this directory. `nix flake check` runs
the same as `proofs-bench_core_proofs`, with overflow checks on and clippy. See [the proofs page](../../docs/handbook/verification/proofs.md).
