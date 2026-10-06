# What the extraction tier assumes

Every theorem under `verification/extraction` depends on Lean's three standard
axioms, `propext`, `Classical.choice` and `Quot.sound`, and on nothing else
except the entries below. `tools/ratchets/stated_axioms.py` fails the build when
an axiom appears that is not listed here, so this file cannot silently fall
behind the corpus.

None of these is a proof axiom. Nothing here asserts that a theorem is true. They
are either models of code outside the kernel that Aeneas declines to translate,
or one mechanised decision procedure. The distinction matters: a proof axiom
would let us prove anything, while an opaque model only means a theorem says
nothing about what that model does.

The core corpus under `verification/lean` carries none of this.
`proof-corpus-root.sh` refuses to emit a root when any axiom outside the standard
three is present, so the release identity is unaffected by everything below.

## The decision procedure

**`<theorem>._native.bv_decide.ax_N`**, 11 theorems.

`bv_decide` reduces a bitvector goal to SAT, runs a solver, and checks the
resulting LRAT certificate. The check runs as compiled code rather than in the
Lean kernel, so each use generates an axiom of the form
`Std.Tactic.BVDecide.Reflect.verifyBVExpr <expr> <cert> = true`. `verifyBVExpr`
is a function; the axiom asserts that evaluating it returned true.

What is trusted is the certificate checker and the compiler that built it. The
SAT solver is not trusted, because its certificate is checked. This is the same
class of trust as `native_decide`.

It is used where a property holds for every value of a machine word and no
smaller statement would do, for example that computing a borrow as
`a ^ ((a ^ b) | ((a - b) ^ b))` decides unsigned less-than on all 2^64 pairs.

## Atomics

**`core.sync.atomic.*`**, the largest group: `Atomic` itself, the `Align1`,
`Align4` and `Align8` markers, and `new`, `load`, `store`, `fetch_add`,
`fetch_sub`, `fetch_or` and `compare_exchange_weak` across the `Bool`, `U8`,
`U32`, `U64` and `Usize` instantiations.

Aeneas does not model shared mutable state, so every atomic is opaque. A theorem
about a function that loads an atomic says what the function does with whatever
the load returned, and says nothing about ordering, visibility or races.

This is a real limit and it is why the kernel's concurrency claims are not made
here. Where a property depends on what two processors observe, the statement
belongs in the core corpus over an explicit model, as
`Nonos.PerCpuAsid` and `Nonos.ApBringup` do.

**`core.sync.atomic.compiler_fence`** and its re-export
**`nonos_ct.core.sync.atomic.compiler_fence`** are in this group but worth naming
separately. The fence emits no instruction and cannot fail; it constrains the
compiler, not the machine. Being opaque here is exactly why a value-level proof
cannot see the difference between a constant-time comparison that has one and one
that does not, which `NonosExtraction.CtEq` states rather than works around.

## Processor identification

**`core.core_arch.x86.cpuid.__cpuid`** and
**`core.core_arch.x86.cpuid.__cpuid_count`**, 10 uses.

What `CPUID` returns is the processor's business. The theorems that depend on
these say what the kernel does with the result: which bit of `edx` a Spectre
probe reads, and that the six probes read six distinct bits. They do not and
cannot say that a given part sets a given bit.

## Arithmetic and allocation left opaque

- **`core.num.U8.wrapping_neg`**, 2 uses. Wrapping negation of a byte.
- **`core.num.Usize.saturating_mul`**, 2 uses. Saturating multiplication.
- **`core.hint.spin_loop`**, 1 use. A scheduling hint that emits a pause
  instruction and has no value.
- **`alloc.string.String.new`**, 2 uses. Allocation, which Aeneas models as
  opaque because the allocator is not in the extracted set.

- **`core.num.U16.wrapping_neg`**, 2 uses. Wrapping negation of a 16-bit word.
- **`core.num.U64.wrapping_neg`**, 1 use. Wrapping negation of a 64-bit
  word, in `ct_is_zero_u64`. The theorem that reads it,
  `ct_is_zero_u64_is_one_exactly_at_zero`, takes the model `-x` modulo `2^64`
  as a stated hypothesis rather than relying on the axiom.
- **`core.num.U64.count_ones`**, 1 use. Population count.
- **`core.num.Usize.div_ceil`**, 1 use. Division rounding up.
- **`core.option.Option.map`**, 1 use, **`core.result.Result.is_err`**, 1 use,
  and **`core.slice.Slice.first`**, 2 uses. Library combinators Aeneas has no
  model for in these crates.
- **`core.ops.range.Range.Insts.CoreIterTraitsIteratorIterator.collect`**, 1
  use. Collecting a range into a container.
- **`alloc.vec.Vec.is_empty`**, 5 uses.
- **`alloc.collections.btree.map.BTreeMap`**, 4 uses, with
  **`alloc.collections.btree.map.BTreeMapKVGlobal.new`**, 2 uses, and
  **`alloc.collections.btree.map.BTreeMapKVGlobal.Insts.CoreDefaultDefault.default`**,
  1 use. The map type and its constructors, opaque for the same reason as
  `String::new`: the allocator is not in the extracted set.

Each only declares that the function exists, with no equation about what it
returns, so a theorem that reaches one says nothing about that call's result.

## Kernel code left opaque

**`memory.addr.phys.PhysAddr.Insts.CoreCmpPartialOrdPhysAddr.ge`**, 2 uses.
This one is not the standard library. It is the kernel's own `>=` on `PhysAddr`,
which the extraction of `memory::frame_alloc::types::range` did not bring in, so
Aeneas declared it without a body.

It is in the profile of two theorems only, and both are wrapper theorems:
`the_framerange_frames_remaining_wrapper_is_its_method` and
`the_framerange_is_exhausted_wrapper_is_its_method`. No property about the order
of physical addresses rests on it, and none is claimed. `frames_remaining` and
`is_exhausted` stay counted as trivial until the comparison is extracted.

## Standard-library calls in the ELF bounds check

`program_header_bounds` makes four calls Aeneas has no model for, and each is
emitted into `Elf.lean` as an opaque axiom. They are in the profile of every
theorem in `NonosExtraction.ElfBoundsRefinement`, because they are in the
function's definition, 5 uses each.

- **`Usize.Insts.CoreConvertTryFromU64TryFromIntError.try_from`**
  (`nonos_elf.Usize.Insts.CoreConvertTryFromU64TryFromIntError.try_from`).
  `usize::try_from(u64)`. Aeneas models the same conversion for other widths as
  `core.num.tryFromUScalar`; this instance is missing from its name table.
- **`core.result.Result.map_err`** (`nonos_elf.core.result.Result.map_err`).
  `Result::map_err`.
- **`core.option.Option.ok_or`** (`nonos_elf.core.option.Option.ok_or`).
  `Option::ok_or`. The policy crate gives it its four-line definition in
  `Policy/FunsExternal.lean`; here it stays opaque.
- **`core.mem.size_of`** (`nonos_elf.core.mem.size_of`). The size of a type, which
  is the compiler's layout decision.

None is a proof axiom. Each declares only that a function of the given type
exists, with no equation about what it returns. A theorem that needs the behaviour
takes it as a named hypothesis in its own statement (`TryFromIsTheConversion`,
`MapErrMapsTheError`, `OkOrIsTheMatch`, `ProgramHeaderIsFiftySixBytes`), so the
dependency is visible where the theorem is used rather than only here. The bound a
caller relies on, `accepted_table_is_inside_the_file`, takes only the weakest: that
`ok_or` never returns a value it was not given.

## Standard-library calls in the TSC conversion and the battery call

`scale` in `sys::timer::tsc::convert` widens to `u128`, multiplies, divides,
and narrows back with a saturating fallback. `sys_battery_status` compares the
firmware's answer, an `Option<bool>`, with `Some(false)`. Aeneas has no model
for three of the library calls this takes, and emits each as an opaque axiom.

- **`U64.Insts.CoreConvertTryFromU128TryFromIntError.try_from`**, 5 uses.
  `u64::try_from(u128)`, the narrowing in `scale`. Aeneas models the same
  conversion for other widths as `core.num.tryFromUScalar`; this instance is
  missing from its name table, as `usize::try_from(u64)` is above.
- **`core.result.Result.unwrap_or`**, 5 uses. `Result::unwrap_or`, which turns
  a failed narrowing into `u64::MAX`.
- **`core.option.Option.Insts.CoreCmpPartialEqOption.eq`**, 1 use.
  `PartialEq` for `Option<bool>`, in `sys_battery_status`.

The uses are the wrapper theorems of `SysTimerTscConvertRefinement` (five
conversions that call `scale`) and of `SyscallMicrokernelBatteryRefinement`,
which reach the calls only through the definitions they unfold to `rfl`. None
is a proof axiom: each declares only that a function of the given type exists.
No theorem claims what `scale` returns on overflow or what the battery call
returns, and none will until these have models or the claim names the behaviour
as a hypothesis.

## Adding one

If a new axiom appears, the gate fails and the fix is to add it here with what it
models and why it is not a proof axiom. Do not widen the gate to make it pass. If
something ever appears that *is* a proof axiom, that is a hole and the gate
failing is the correct outcome.
