/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

A guest's store, and why it is the only thing a guest's paths can name.

Guests were handed the personality's own view of the filesystem. Nothing had gone
wrong with the capability table, because paths are not capabilities: a guest with
no authority at all still opens files by name, and the name it supplies is
resolved by code holding the personality's authority. Confinement therefore has to
be a property of resolution, not of the token.

So resolution is the only thing that produces a `Key`, every store operation takes
one, and a `Key` carries a path that has already been reduced under the guest's
root. The theorems are about the reduction. `escape_is_refused` is the one that
matters: a `..` at the root is refused rather than clamped, because clamping makes
`/linux/../etc/passwd` resolve to `/linux/etc/passwd`, which is a different file
than the guest asked for and a worse failure than an error. `root_is_a_prefix` is
the confinement statement, and it holds by construction: the reduction only ever
computes the part of the path below the root, so the root cannot be reduced away.

`depth_is_bounded` is here because a resolver that can grow a path without bound
is a memory bug, and a path is guest input.
-/

namespace Nonos.StoreRoot

/-! ### Paths -/

/-- A path component. Names are identifiers here; what matters is that they are
    distinguishable and that `..` and `.` are not among them, which is what
    tokenising before resolving buys. -/
inductive Step where
  | here
  | up
  | down (name : Nat)
  deriving DecidableEq, Repr

/-- The reduction, carrying the part of the path below the root.

    The accumulator is innermost-first. `up` at depth zero has nowhere to go and
    is refused; it is not clamped to the root. -/
def walk (acc : List Nat) : List Step → Option (List Nat)
  | [] => some acc
  | .here :: rest => walk acc rest
  | .up :: rest =>
    match acc with
    | [] => none
    | _ :: below => walk below rest
  | .down n :: rest => walk (n :: acc) rest

/-- Resolution: start at the root, reduce, and hand back the whole path with the
    root in front. -/
def resolve (root : List Nat) (steps : List Step) : Option (List Nat) :=
  (walk [] steps).map (fun rel => root ++ rel.reverse)

/-! ### Confinement -/

/-- Every resolved path begins with the root.

    This is the confinement property, and it is true by construction rather than
    by a check: the reduction computes the part below the root and never sees the
    root at all, so there is no path through it that removes a root component. -/
theorem root_is_a_prefix (root : List Nat) (steps : List Step) (p : List Nat)
    (h : resolve root steps = some p) : ∃ rel, p = root ++ rel := by
  unfold resolve at h
  cases hw : walk [] steps with
  | none => rw [hw] at h; simp at h
  | some rel =>
    rw [hw] at h
    simp at h
    exact ⟨rel.reverse, h.symm⟩

/-- A `..` with nothing below it is refused. -/
theorem up_at_the_root_is_refused (rest : List Step) : walk [] (.up :: rest) = none := by
  simp [walk]

/-- And therefore so is any path that tries to leave, however it is spelled: a
    run of `..` from the root refuses at the first one, not at the last. -/
theorem escape_is_refused (k : Nat) (rest : List Step) :
    walk [] (List.replicate (k + 1) .up ++ rest) = none := by
  rw [List.replicate_succ]
  simp only [List.cons_append]
  exact up_at_the_root_is_refused _

/-- Resolution refuses it too, rather than producing a path under the root that
    the guest did not ask for. Clamping would have resolved
    `/linux/../etc/passwd` to `/linux/etc/passwd`: a real file, a wrong answer,
    and no error anywhere. -/
theorem resolve_refuses_an_escape (root : List Nat) (k : Nat) (rest : List Step) :
    resolve root (List.replicate (k + 1) .up ++ rest) = none := by
  unfold resolve
  rw [escape_is_refused k rest]
  rfl

/-- A `..` that has somewhere to go goes there: the reduction is a reduction and
    not a refusal of every `..`, so ordinary relative paths work. -/
theorem up_below_the_root_pops (n : Nat) (below : List Nat) (rest : List Step) :
    walk (n :: below) (.up :: rest) = walk below rest := by
  simp [walk]

/-- `.` changes nothing, so a path may contain as many as it likes. -/
theorem here_is_a_no_op (acc : List Nat) (rest : List Step) :
    walk acc (.here :: rest) = walk acc rest := by
  simp [walk]

/-! ### Bounds -/

/-- The reduction never grows the path by more than the number of steps it was
    given. A path is guest input, so an unbounded reduction is a memory bug
    before it is a confinement bug. -/
theorem depth_is_bounded (acc : List Nat) (steps : List Step) (rel : List Nat)
    (h : walk acc steps = some rel) : rel.length ≤ acc.length + steps.length := by
  induction steps generalizing acc rel with
  | nil =>
    unfold walk at h
    injection h with h
    subst h
    simp
  | cons s rest ih =>
    cases s with
    | here =>
      rw [here_is_a_no_op] at h
      have := ih acc rel h
      simp
      omega
    | up =>
      cases acc with
      | nil => rw [up_at_the_root_is_refused] at h; simp at h
      | cons n below =>
        rw [up_below_the_root_pops] at h
        have := ih below rel h
        simp
        omega
    | down n =>
      unfold walk at h
      have := ih (n :: acc) rel h
      simp at this
      simp
      omega

/-- And a resolved path is the root plus at most as many components as the guest
    supplied steps. -/
theorem resolved_length_is_bounded (root : List Nat) (steps : List Step) (p : List Nat)
    (h : resolve root steps = some p) : p.length ≤ root.length + steps.length := by
  unfold resolve at h
  cases hw : walk [] steps with
  | none => rw [hw] at h; simp at h
  | some rel =>
    rw [hw] at h
    simp at h
    subst h
    have := depth_is_bounded [] steps rel hw
    simp at this
    simp
    omega

/-! ### The key -/

/-- The token a store operation requires. It carries a path that has already been
    reduced under a root, and the only way to obtain one is `mint`, which is the
    only caller of `resolve`. -/
structure Key where
  root : List Nat
  path : List Nat
  deriving DecidableEq, Repr

/-- The one producer. -/
def mint (root : List Nat) (steps : List Step) : Option Key :=
  (resolve root steps).map (fun p => ⟨root, p⟩)

/-- Every key that exists carries a path under its own root. A store operation
    that takes a key and nothing else therefore cannot be asked to touch anything
    outside it, and does not need to check: there is no key that names a path
    outside its root. -/
theorem every_key_is_confined (root : List Nat) (steps : List Step) (k : Key)
    (h : mint root steps = some k) : ∃ rel, k.path = k.root ++ rel := by
  unfold mint at h
  cases hr : resolve root steps with
  | none => rw [hr] at h; simp at h
  | some p =>
    rw [hr] at h
    simp at h
    subst h
    exact root_is_a_prefix root steps p hr

/-- A key's root is the root it was minted against, so a caller cannot mint under
    one root and present the key as being under another. -/
theorem key_remembers_its_root (root : List Nat) (steps : List Step) (k : Key)
    (h : mint root steps = some k) : k.root = root := by
  unfold mint at h
  cases hr : resolve root steps with
  | none => rw [hr] at h; simp at h
  | some p =>
    rw [hr] at h
    simp at h
    subst h
    rfl

/-- An escaping path mints no key, so there is nothing to hand to a store
    operation and nothing for a store operation to validate. -/
theorem escape_mints_no_key (root : List Nat) (k : Nat) (rest : List Step) :
    mint root (List.replicate (k + 1) .up ++ rest) = none := by
  unfold mint
  rw [resolve_refuses_an_escape]
  rfl

/-- A store operation, which takes nothing but a key. The signature is the
    confinement: there is no path argument, so there is no path to check. -/
def storeOpen (k : Key) : List Nat := k.path

/-- And so every open lands under the root the key was minted against. -/
theorem every_open_is_under_the_root (root : List Nat) (steps : List Step) (k : Key)
    (h : mint root steps = some k) : ∃ rel, storeOpen k = root ++ rel := by
  obtain ⟨rel, hrel⟩ := every_key_is_confined root steps k h
  rw [key_remembers_its_root root steps k h] at hrel
  exact ⟨rel, hrel⟩

end Nonos.StoreRoot
