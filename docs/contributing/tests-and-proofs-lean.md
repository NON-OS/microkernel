# Tests and proofs: Lean

Part of [Tests and proofs](tests-and-proofs.md).

## Lean

`verification/lean` is a Lean 4 development with no mathlib. Its
`defaultTargets` names one library, `Nonos`
(`verification/lean/lakefile.toml:2-5`), and `verification/lean/lean-toolchain`
pins Lean 4.15.0. At this commit it has 190 modules and 1499 `theorems`, and
`sorry_count` is 0 (`verification/evidence/EVIDENCE.json:2176-2181`). The `lean`
job of the `verify` job runs `lake build`, then fails if the axiom profile shows `sorryAx`
(`.github/workflows/ci-verify.yml`).

```
cd verification/lean
lake build
```

Not tested in this release.

## Extraction and Verus

Charon and Aeneas lower Rust MIR into Lean. At this commit 976 functions are
extracted; 490 are `substantive`, with a theorem about their behaviour, and 486
are `trivial`, with only the theorem that a generated wrapper is the method it
forwards to (`verification/evidence/EVIDENCE.json:28-32`).
`tools/ratchets/proven_functions.py` holds the proven count to a floor that may
only rise, except when the code a proof covered is deleted.

`verification/verus` proves theorems about capability bit operations, page-table
permission encoding and IPC length guards as restated in its own spec functions.
It mounts no kernel file, so a kernel change does not reach it;
`verification/README.md` says so. It has 5 `source_files`
(`verification/evidence/EVIDENCE.json:2187`).

## Fuzzing

`ci-fuzz.yml` runs cargo-fuzz over the thirteen targets `fuzz/targets.json`
lists under `ci`, from `v4_parse` in `nonos-attest-path` to `linux_policy` in
`userland/market_proto`: two minutes each on every pull request and push, and
30 minutes each every night, unless a manual run asks for another length
(`.github/workflows/ci-fuzz.yml`). Each target's corpus is kept in the Actions
cache between runs. A crash fails the run with the command that replays it, and
the log and the crashing input, from the crate's `artifacts` directory, are kept
with the run; [fuzz/README.md](../../fuzz/README.md) describes the records.

```
cd userland/kernel_proofs/fuzz
cargo fuzz run elf_header corpus/elf_header -- -max_total_time=60
```

Not tested in this release.

## See also

- [Code style](code-style.md)
- [Review](review.md)
- [CI](../build/ci.md)
- [The Nix flake](../build/nix-flake.md)
- [Reproducible builds](../build/reproducible-builds.md)
- [Contributing](README.md)
