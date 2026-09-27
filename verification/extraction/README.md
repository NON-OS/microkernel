# Rust to Lean extraction

This directory makes the refinement between the Lean specification and the
kernel machine-checked instead of nominal. The pipeline lowers the real
kernel source to Lean through the compiler's own MIR and proves the security
semantics about the result:

```
src/capabilities/bits.rs          the code that ships
        |  rustc MIR, via Charon (#[path] include, never a copy)
        v
caps/caps.llbc                    the borrow-checked IR of that exact code
        |  Aeneas, Lean backend
        v
lean/NonosExtraction/Caps.lean    the same functions as Lean definitions
        |  lean/NonosExtraction/Refinement.lean
        v
Grants / capsOf semantics         the objects the lattice theorems quantify over
```

`Caps.lean` is generated, never edited; regenerating it from an unchanged
kernel is byte-stable, and any kernel change shows up as a diff here. The
theorems in `Refinement.lean` prove the extracted `has_capability`,
`add_capability`, and `remove_capability` are total and compute exactly the
`capsOf` denotation from `verification/lean/Nonos/CapabilityBits.lean`, so
every lattice theorem proven there now holds of the extracted-from-real-Rust
definitions by transitivity.

## Scope, honestly stated

Extraction covers the pure safe decision cores, nine crates and ninety-two
functions. Each crate mirrors the kernel's module paths with `#[path]` so the
included files find each other at the paths they already use, and each one holds
no lock, no atomic and no hardware access.

| crate | what it carries | proven in |
| --- | --- | --- |
| `caps/` | capability bit operations and `Capability::bit`, the resolver `select_caps`, the folder `fold_caps`, the delegation expiry meet, the quota comparison, the resource nonce composition, the chain depth bound | `Refinement`, `CapsComplete`, `CapsCoreRefinement` |
| `policy/` | the user-copy range policy `check_range` with the exact error variant on every rejecting path, and the page-permission encoding `to_pte_flags` and `is_wx_violation` | `PolicyRefinement` |
| `irq/` | the MSI-X bind validator | `IrqRefinement` |
| `vectors/` | the interrupt vector classification and the two conversions between a line and a vector | `VectorsRefinement` |
| `signal/` | the signal delivery policy, nine predicates | `SignalRefinement` |
| `ct/` | both constant-time comparison implementations, the selectors, the lookups and the arithmetic helpers | `CtRefinement`, `CtPrimitivesRefinement` |
| `iommu/` | the second-level page-table and context-table encodings, the indexing, and the address width the AGAW chooses | `IommuRefinement` |
| `paging/` | both page-descriptor backends, x86_64 and aarch64, read and write | `PagingRefinement` |
| `elf/` | the relocation range check and the relocation type allowlist | `ElfRefinement` |

Some of what is extracted is proven wrong rather than proven right, and the file
headers say which. `CtRefinement` proves the two constant-time comparisons
disagreed and keeps the old shape named so the regression cannot come back;
`PagingRefinement` proves the aarch64 table builder ignores its
`user_accessible` argument; `VectorsRefinement` proves `irq_to_vector` fails
above line 223. A refinement file that only proved agreement would be hiding
those.

Three things are extracted and not fully proven, said here rather than left to be
discovered. `program_header_bounds` in `elf/` is regenerated and diffed by CI, so
a change to it shows up, but no theorem covers it: a witness needs a fifteen-field
header and a slice, and `?` desugars into `ControlFlow` over an opaque
`Option::ok_or`. `ct_clz_u64` is a chain of six nested selects that does not close
by reduction, and `ct_select_usize` goes through a cast the kernel will not reduce
past. The first is covered by
`kernel_proofs::elf_tests::program_header_table_never_overflows_or_escapes_the_file`,
a host test that crafts three hundred thousand headers and asserts the table
neither overflows nor leaves the file; the other two were sampled against their
references on twenty thousand random words with no disagreement. All three are
tests, not proofs, and none is described as one.

`select_caps` was written with iterator adapters, which are outside Aeneas's
supported fragment, so it could not be extracted at all; it takes its table as an
argument and walks it now, and `CapsComplete.lean` proves the resulting loop
against a specification. `Capability::all` is a static, so Charon leaves it
opaque: the resolver is proven faithful to the table it is handed, and that the
table is the whole enumeration is the `capability_table!` macro's job, checked at
compile time by `guard.rs`. The unsafe hardware glue is out of extraction scope by
design and remains under Kani and Verus.

One external definition is provided by hand, as Aeneas prescribes for core
functions it treats as opaque: `Option::ok_or`, four lines in
`lean/Policy/FunsExternal.lean`, entering the trusted base with the
documented core semantics. Everything else in `lean/Policy` and
`lean/NonosExtraction/Caps.lean` is generated and never edited.

## Trusted base and pins

- Charon `nightly-2026.07.02` (v0.1.218), Aeneas `nightly-2026.07.06-45061fa`,
  prebuilt release binaries; Charon drives rustc `nightly-2026-06-01`.
- The Aeneas Lean library at the same commit, with Lean `v4.30.0-rc2` and
  mathlib as its dependency. The core `verification/lean` package stays
  core-only on its own pinned toolchain; this project rebuilds it from the
  same sources under rc2 through a lake path dependency.
- The claim "Caps.lean is the real bits.rs" rests on Charon reading the MIR
  of the included file and on Aeneas's translation soundness, the same way
  the host proofs rest on rustc.
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
# are in the `extraction` job of .github/workflows/verify.yml, which regenerates
# all nine and diffs each one for drift, so that job is the source of truth and
# this file does not repeat it.

cd ../lean && lake exe cache get && lake build   # 0 errors == verified

# NonosExtraction.lean is the root: a module missing from its imports is not
# built by the default target and so is checked by nothing.
```
