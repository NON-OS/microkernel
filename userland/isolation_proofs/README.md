# isolation_proofs

Host proofs for isolation_stark:

- an honest histogram proves, and the proof verifies through the engine,
  through nox_verify over the program image, and under
  nonos_proof_abi::isolation::STATEMENT;
- the pinned image and every number beside it are the circuit's;
- claims a witness does not back, and bucket rows changed after the chains
  were laid, never verify.

`cargo run --release --bin isolation_image -- <dir>` writes program.bin and
its MANIFEST from the circuit; `isolation_vector -- vectors` writes the honest
proof and its words that ship in vectors/ for the owner's attest step. Run the tests with
`RUST_TEST_THREADS=1 cargo test --release`.
