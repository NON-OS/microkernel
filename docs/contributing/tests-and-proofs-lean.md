# Tests and proofs: Lean

Part of [Tests and proofs](tests-and-proofs.md).

## Lean

`verification/lean` is a Lean 4 development with no mathlib. Its
`defaultTargets` names one library, `Nonos`
(`verification/lean/lakefile.toml:2-5`), and `verification/lean/lean-toolchain`
pins Lean 4.15.0. At this commit it has 190 modules and 1499 `theorems`, and
`sorry_count` is 0 (`verification/evidence/EVIDENCE.json:2176-2181`). The `lean`
workflow runs `lake build`, then fails if the axiom profile shows `sorryAx`
(`.github/workflows/lean.yml:49-60`).

```
cd verification/lean
lake build
```

Not tested in this release.

## Extraction and Verus

Charon and Aeneas lower Rust MIR into Lean. At this commit 979 functions are
extracted; 492 are `substantive`, with a theorem about their behaviour, and 487
are `trivial`, with only the theorem that a generated wrapper is the method it
forwards to (`verification/evidence/EVIDENCE.json:28-32`).
`tools/ratchets/proven_functions.py` holds the proven count to a floor that may
only rise.

`verification/verus` proves theorems about capability bit operations, page-table
permission encoding and IPC length guards as restated in its own spec functions.
It mounts no kernel file, so a kernel change does not reach it;
`verification/README.md` says so. It has 5 `source_files`
(`verification/evidence/EVIDENCE.json:2187`).

## Fuzzing

`fuzz.yml` runs cargo-fuzz every night on its `cron`, at 01:00 UTC, over the
twelve targets in its `matrix`, from `v4_parse` in `nonos-attest-path` to
`linux_policy` in `userland/market_proto` (`.github/workflows/fuzz.yml:12-68`).
Each target runs for `SECONDS_PER_TARGET`, 1800 seconds unless a manual run asks
for another length (`.github/workflows/fuzz.yml:69-70`). A crash fails the run,
and the log and the crashing input, from the crate's `artifacts` directory, are
kept with the run (`.github/workflows/fuzz.yml:96-104`).

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
