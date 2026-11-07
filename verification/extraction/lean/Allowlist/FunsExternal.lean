/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

What the allowlist extraction does not translate, filled in from
FunsExternal_Template.lean as Aeneas prescribes.

`usize::div_ceil` is defined by the behaviour the Rust standard library
documents: it panics on a zero divisor, and otherwise rounds the quotient up.

The registry index, `syscall::abi::index`, is translated opaque. Its hash
table and its count are built by `const fn` at compile time, and Aeneas has no
model for those statics. Here `COUNT` and `slot_of` are Lean `opaque`
constants: they stand for some count and some lookup, add no axiom, and every
theorem about the allowlist holds whatever they are. The index is tested on
the host instead (kernel_proofs `syscall/index_tests.rs`).
-/

import Aeneas
import Allowlist.Types
open Aeneas Aeneas.Std Result ControlFlow Error

set_option linter.dupNamespace false
set_option linter.hashCommand false
set_option linter.unusedVariables false

open nonos_x_allowlist

/-- [core::num::{usize}::div_ceil] -/
@[rust_fun "core::num::{usize}::div_ceil"]
def core.num.Usize.div_ceil (a b : Std.Usize) : Result Std.Usize := do
  let q ← a / b
  let r ← a % b
  if r = 0#usize then ok q else q + 1#usize

/-- [nonos_x_allowlist::syscall::abi::index::COUNT]: the number of registry
    slots, computed at compile time. -/
opaque syscall.abi.index.COUNT : Result Std.Usize

/-- [nonos_x_allowlist::syscall::abi::index::slot_of]: the registry slot of a
    syscall number, by the compile-time hash table. -/
opaque syscall.abi.index.slot_of : Std.U64 → Result (Option Std.Usize)
