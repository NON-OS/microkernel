# clock_resolve_proofs

A host proof for the kernel's boot wall-clock source picker,
`src/sys/clock/resolve.rs`, included through `#[path]`. It decides which
TSC-frequency source and which epoch source the clock uses at boot, in
priority order, and asks a lower-priority source only when a higher one
gave nothing. `tests.rs` holds the priority and laziness rules.

Run: `cargo test` in this directory. Timekeeping is described in
[Interrupts and time](../../docs/handbook/kernel/interrupts-and-time.md).
