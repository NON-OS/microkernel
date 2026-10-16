# seal_proofs

Host tests for the kernel's family seal, `src/process/foreign/seal`: the trace
chain K, the histogram H and its commitment Hc, the measurement chain M, a
family's open calls and how each closes, and the 240-byte record a sealed
family leaves. The files are mounted unchanged by `#[path]`. What the kernel
writes is read back by `userland/nonos_proof_abi`, so a byte one side writes
and the other reads differently fails here.

The files the seal reaches through are held too: the parked table its
answers come by, the frame a supervisor reads (field for field against
`nonos_libc`), the guest registry and the errno table. Each mounted item is
read by a test; no lint is switched off.

Run with `cargo test --release` from this directory.
