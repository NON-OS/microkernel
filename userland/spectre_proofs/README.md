# spectre_proofs

Host tests for the aarch64 speculation decision, mounted by `#[path]` from
`src/arch/aarch64/security/spectre/`:

- `decide.rs` and `history.rs`: the plan for one CPU from its ID registers
  (CSV2, CSV3, SSBS, ECBHB, CLRBHB, E0PD), its MIDR and the firmware's
  SMCCC_ARCH_FEATURES answers. Walked over every combination of the fields
  at each value the decision tells apart, ten core classes and all 27
  firmware answers: ID registers outrank firmware, firmware outranks the
  core lists, "vulnerable" only where nothing could be done, and the loop
  only with a published count.
- `cores.rs`: named rows for Cortex-A53, A72, A73, A75 and A76, and another
  implementer that is not guessed at.
- `answer.rs`: the ARCH_FEATURES return values.
- `entry_word.rs`: the word the EL0 vectors read runs each plan as decided,
  and merging CPUs' words is commutative, associative, idempotent and gives
  exactly the stronger call, the longer loop and CLRBHB if either needed it.

Run with `RUST_TEST_THREADS=1 cargo test --release --config
profile.release.overflow-checks=true` from this directory.
