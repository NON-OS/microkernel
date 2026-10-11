# seal_proofs

Host tests for the kernel's family seal, `src/process/foreign/seal`: the trace
chain K, the histogram H and its commitment Hc, the measurement chain M, a
family's open calls and how each closes, and the 240-byte record a sealed
family leaves. The files are mounted unchanged by `#[path]`. What the kernel
writes is read back by `userland/nonos_proof_abi`, so a byte one side writes
and the other reads differently fails here.

Run with `cargo test --release` from this directory.
