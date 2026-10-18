# Rust to Lean extraction: Trusted base and pins

Part of [Rust to Lean extraction](README.md).

## Trusted base and pins

- Charon `nightly-2026.07.02` (v0.1.218), Aeneas `nightly-2026.07.06-45061fa`,
  prebuilt release binaries; Charon drives rustc `nightly-2026-06-01`.
- The Aeneas Lean library at the same commit, with Lean `v4.30.0-rc2` and
  mathlib as its dependency. The core `verification/lean` package stays
  core-only on its own pinned toolchain; this project rebuilds it from the same
  sources under rc2 through a lake path dependency.
- The claim "Caps.lean is the real bits.rs" rests on Charon reading the MIR of
  the included file and on Aeneas's translation soundness, the same way the host
  proofs rest on rustc.
- The refinement theorems depend on Lean's standard axioms only (propext,
  Classical.choice, Quot.sound); `#print axioms` shows no sorry. The Aeneas
  library itself carries sorries in modules these proofs do not use (Slice,
  StringIter); they do not enter the axiom profile of any theorem here.

## Reproduce

```sh
cd verification/extraction/caps
charon cargo --preset=aeneas \
  --start-from 'nonos_caps::capabilities::bits::has_capability' \
  --start-from 'nonos_caps::capabilities::bits::add_capability' \
  --start-from 'nonos_caps::capabilities::bits::remove_capability' \
  --start-from 'nonos_caps::capabilities::bits::fold_caps' \
  --start-from 'nonos_caps::capabilities::bits::select_caps' \
  --dest-file caps.llbc
aeneas -backend lean caps.llbc -dest ../lean/NonosExtraction

cd ../policy
charon cargo --preset=aeneas \
  --start-from 'nonos_policy::usercopy::policy::check_range' \
  --start-from 'nonos_policy::to_pte_flags' \
  --start-from 'nonos_policy::is_wx_violation' \
  --dest-file policy.llbc
aeneas -backend lean -split-files policy.llbc -dest ../lean/Policy
# FunsExternal.lean is hand-written from FunsExternal_Template.lean; do not
# overwrite it.

# Every other crate follows the same two commands. The exact --start-from sets
# are in crates.json. The `extraction` job of .github/workflows/verify.yml runs
# tools/extraction/regen.py, which regenerates every crate listed there and
# diffs each one for drift, so that file is the source of truth and this one
# does not repeat it.

cd ../lean && lake exe cache get && lake build   # 0 errors == verified

# lakefile.toml builds every module under NonosExtraction by glob, so a module
# missing from NonosExtraction.lean's imports is still built and checked.
```
