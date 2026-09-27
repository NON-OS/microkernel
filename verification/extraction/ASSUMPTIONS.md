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

## Adding one

If a new axiom appears, the gate fails and the fix is to add it here with what it
models and why it is not a proof axiom. Do not widen the gate to make it pass. If
something ever appears that *is* a proof axiom, that is a hole and the gate
failing is the correct outcome.
