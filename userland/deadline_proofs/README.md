# deadline_proofs

Host tests for the libc's monotonic `Deadline` (`userland/libc/src/time/deadline.rs`),
mounted by `#[path]` with a settable stand-in for the uptime syscall, so its
comparison and its live paths run unchanged, including a clock read that fails
when the deadline is set or when it is checked. 7 tests.

Run with `cargo test --release` from this directory. `nix flake check` runs
the same as `proofs-deadline_proofs`, with overflow checks on and clippy. See [the proofs page](../../docs/handbook/verification/proofs.md).
