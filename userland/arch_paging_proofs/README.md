# arch_paging_proofs

Host proofs for the page-table descriptor encoders of both backends, compiled
side by side from the kernel tree by `#[path]`: `src/arch/paging/descriptor`
(the x86_64 encoder, the aarch64 bits, build and read) and the aarch64 memory
attribute kinds. A boot only exercises one of them, so this is where the two
are compared. The crate also checks the civil calendar conversion in
`src/sys/clock/civil` in both directions, which the CMOS clock and the PL031
each use one way. 17 tests, and 7 Kani proofs (`cargo kani`) that the `proof-crates-kani` job
of `verify.yml` runs.

Run with `cargo test --release` from this directory. `nix flake check` runs
the same as `proofs-arch_paging_proofs`, with overflow checks on and clippy. See [the proofs page](../../docs/handbook/verification/proofs.md).
