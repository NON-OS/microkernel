/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

The library calls the capability table makes, filled in from
FunsExternal_Template.lean as Aeneas prescribes. Aeneas has no model for these
five, so each is defined here by the behaviour the Rust standard library
documents, and enters the trusted base as a definition, not an axiom:

- `Option::map_or(d, f)`: `d` on `None`, `f(x)` on `Some(x)`.
- `Option::or_else(f)`: the option itself when `Some`, `f()` when `None`.
- `Iterator::any(f)` and `Iterator::all(f)` on a slice iterator: `f` is called
  on each remaining element in order, stopping at the first that decides the
  answer; the iterator is left past the last element examined.
- `Vec::is_empty`: the length is zero.

The kernel clock is the one item left opaque. It is declared without a body
in the extraction root, so here it is a constant of an inhabited type, which
asserts only that a reading exists. A process token carries no expiry, and
`CapTableRefinement` proves its checks never consult the clock.
-/

import Aeneas
import CapTable.Types
open Aeneas Aeneas.Std Result ControlFlow Error

set_option linter.dupNamespace false
set_option linter.hashCommand false
set_option linter.unusedVariables false

open nonos_x_cap_table

/-- [core::option::{core::option::Option<T>}::map_or] -/
@[rust_fun "core::option::{core::option::Option<@T>}::map_or"]
def core.option.Option.map_or
  {T : Type} {U : Type} {F : Type} (inst : core.ops.function.FnOnce F T U) :
  Option T → U → F → Result U
  | none, d, _ => ok d
  | some x, _, f => inst.call_once f x

/-- [core::option::{core::option::Option<T>}::or_else] -/
@[rust_fun "core::option::{core::option::Option<@T>}::or_else"]
def core.option.Option.or_else
  {T : Type} {F : Type} (inst : core.ops.function.FnOnce F Unit (Option T)) :
  Option T → F → Result (Option T)
  | some x, _ => ok (some x)
  | none, f => inst.call_once f ()

/-- `any` over the elements `l`, the iterator's position counted in `n`. -/
def sliceAny {T : Type} {F : Type} (inst : core.ops.function.FnMut F T Bool) :
    F → List T → Nat → Result (Bool × Nat)
  | _, [], n => ok (false, n)
  | f, x :: xs, n => do
      let (b, f') ← inst.call_mut f x
      if b then ok (true, n + 1) else sliceAny inst f' xs (n + 1)

/-- `all` over the elements `l`, the iterator's position counted in `n`. -/
def sliceAll {T : Type} {F : Type} (inst : core.ops.function.FnMut F T Bool) :
    F → List T → Nat → Result (Bool × Nat)
  | _, [], n => ok (true, n)
  | f, x :: xs, n => do
      let (b, f') ← inst.call_mut f x
      if b then sliceAll inst f' xs (n + 1) else ok (false, n + 1)

/-- [core::slice::iter::Iter::all] -/
@[rust_fun
  "core::slice::iter::{core::iter::traits::iterator::Iterator<core::slice::iter::Iter<'a, @T>, &'a @T>}::all"]
def core.slice.iter.Iter.Insts.CoreIterTraitsIteratorIteratorSharedAT.all
  {T : Type} {F : Type} (inst : core.ops.function.FnMut F T Bool)
  (it : core.slice.iter.Iter T) (f : F) : Result (Bool × (core.slice.iter.Iter T)) := do
  let (b, n) ← sliceAll inst f (it.slice.val.drop it.i) it.i
  ok (b, { it with i := n })

/-- [core::slice::iter::Iter::any] -/
@[rust_fun
  "core::slice::iter::{core::iter::traits::iterator::Iterator<core::slice::iter::Iter<'a, @T>, &'a @T>}::any"]
def core.slice.iter.Iter.Insts.CoreIterTraitsIteratorIteratorSharedAT.any
  {T : Type} {F : Type} (inst : core.ops.function.FnMut F T Bool)
  (it : core.slice.iter.Iter T) (f : F) : Result (Bool × (core.slice.iter.Iter T)) := do
  let (b, n) ← sliceAny inst f (it.slice.val.drop it.i) it.i
  ok (b, { it with i := n })

/-- [alloc::vec::{alloc::vec::Vec<T>}::is_empty] -/
@[rust_fun "alloc::vec::{alloc::vec::Vec<@T>}::is_empty"]
def alloc.vec.Vec.is_empty {T : Type} (A : Type) (v : alloc.vec.Vec T) : Result Bool :=
  ok v.val.isEmpty

/-- [nonos_x_cap_table::time::timestamp_millis]: the kernel clock, declared
    without a body in the extraction root. -/
opaque time.timestamp_millis : Result Std.U64
