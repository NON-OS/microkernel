# Rust to Lean extraction: Scope, honestly stated

Part of [Rust to Lean extraction](README.md).

## Scope, honestly stated

Extraction covers the pure safe decision cores. The table below is the nine
hand-built crates this pipeline started with. `crates.json` now lists 285 crates
with 978 entry points: 13 hand-built, 129 made by
`tools/extraction/closure_crate.py` under `tree/` and 143 made by
`tools/extraction/sweep.py` under `sweep/`. `tools/ratchets/proven_functions.py`
counts all 978 as carrying a proof, 492 of them with a property beyond the
theorem that the forwarding wrapper equals its method. Each crate mirrors the
kernel's module paths with `#[path]` so the included files find each other at
the paths they already use, and each one holds no lock, no atomic and no
hardware access.

| crate | what it carries | proven in |
| --- | --- | --- |
| `caps/` | capability bit operations and `Capability::bit`, the resolver `select_caps`, the folder `fold_caps`, the delegation expiry meet, the quota comparison, the resource nonce composition, the chain depth bound | `Refinement`, `CapsComplete`, `CapsFoldRefinement`, `CapsCoreRefinement` |
| `policy/` | the user-copy range policy `check_range` with the exact error variant on every rejecting path, and the page-permission encoding `to_pte_flags` and `is_wx_violation` | `PolicyRefinement` |
| `irq/` | the MSI-X bind validator | `IrqRefinement` |
| `vectors/` | the interrupt vector classification and the two conversions between a line and a vector | `VectorsRefinement` |
| `signal/` | the signal delivery policy, nine predicates | `SignalRefinement` |
| `ct/` | both constant-time comparison implementations, the selectors, the lookups and the arithmetic helpers | `CtRefinement`, `CtPrimitivesRefinement` |
| `iommu/` | the second-level page-table and context-table encodings, the indexing, and the address width the AGAW chooses | `IommuRefinement` |
| `paging/` | both page-descriptor backends, x86_64 and aarch64, read and write | `PagingRefinement` |
| `elf/` | the relocation range check, the relocation type allowlist and the program-header table bounds | `ElfRefinement`, `ElfBoundsRefinement` |

Some of what is extracted is proven wrong rather than proven right, and the file
headers say which. `CtRefinement` proves the two constant-time comparisons
disagreed and keeps the old shape named so the regression cannot come back;
`PagingRefinement` proves the aarch64 table builder honours its
`user_accessible` argument and keeps the old shape, which ignored it, named with
the theorem that says so; `VectorsRefinement` proves `irq_to_vector` fails above
line 223. A refinement file that only proved agreement would be hiding those.

Two things are extracted and not fully proven, said here rather than left to be
discovered. `ct_clz_u64` is a chain of six nested selects that does not close by
reduction, and `ct_select_usize` goes through a cast the kernel will not reduce
past. Both were sampled against their references on twenty thousand random words
with no disagreement. That is a test, not a proof, and neither is described as
one.

`program_header_bounds` in `elf/` used to be a third. `ElfBoundsRefinement`
proves that an accepted table ends inside the image, assuming only that
`Option::ok_or` returns no value it was not given, and that on every header
whose offset fits a `usize` the function returns the verdict of the
`Nonos.ElfPhdr` model, assuming the documented behaviour of the four
standard-library calls Aeneas leaves opaque. Those assumptions are hypotheses in
the theorems, and the four calls are registered in `ASSUMPTIONS.md`. The host
test
`kernel_proofs::elf_tests::program_header_table_never_overflows_or_escapes_the_file`
still runs.

`select_caps` was written with iterator adapters, which are outside Aeneas's
supported fragment, so it could not be extracted at all; it takes its table as
an argument and walks it now, and `CapsComplete.lean` proves the resulting loop
against a specification. `Capability::all` is a static, so Charon leaves it
opaque: the resolver is proven faithful to the table it is handed, and that the
table is the whole enumeration is the `capability_table!` macro's job, checked
at compile time by `guard.rs`. The unsafe hardware glue is out of extraction
scope by design and remains under Kani and Verus.

One external definition is provided by hand, as Aeneas prescribes for core
functions it treats as opaque: `Option::ok_or`, four lines in
`lean/Policy/FunsExternal.lean`, entering the trusted base with the documented
core semantics. Everything else in `lean/Policy` and
`lean/NonosExtraction/Caps.lean` is generated and never edited.
