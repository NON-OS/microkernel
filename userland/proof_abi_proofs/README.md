# proof_abi_proofs

Host tests for `userland/nonos_proof_abi`, the bytes a NONOS proof travels in:
the kernel's sealed record, the NPRF container, the device proof's scope, the
anchor, the histogram and syscall-set chains, and the verify order.

One test proves a real anonymous device statement bound to a real record, so
the binding, the trust roots and the scope are checked against the device
prover itself. Minutes, and about 2.6 GB. The isolation statement lands with
`isolation_stark`; until then the order is run to its last step, which
refuses a container that has no isolation proof.

Run with `cargo test --release` from this directory.
