# agentproof_proofs

Host proofs for `agentproof_policy`.

- Every refusal of the format, with the line it names and its reason.
- The presets hash as capsule_linux's tests pin them, and an `allow:` box hashes alike
  whatever order its folders are written in.
- Enforcement exactly as written: for 140,000 paths over seven policies, capsule_linux's own
  write rule lets a path be written iff the file's words say so; every box holds exec to the
  box and no network; a home folder shows only when the program has a home.
- The command's exits and output.
- A seeded sweep of 1,000,000 files and every single-bit flip of a good one, held to
  `check::check_file`: a refusal names a line inside the file, or the policy reads back as
  written and capsule_linux reads its word back as the same box with the same hash.
- The fuzz target `policy_file` holds any bytes to the same check.
- `mutants.py` removes each control listed in `mutants.json` in turn; the proofs must fail
  for every one.

```sh
cargo test --release --locked
cargo clippy --all-targets --locked -- -D warnings
python3 mutants.py
cd fuzz && cargo +nightly-2026-01-16 fuzz run policy_file -- -runs=12000000
```
