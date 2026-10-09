/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

The attestation path check, and what an accepted path means.

The bootloader, the capsule spawn gate and secops no longer run a proof system.
Each rebuilds a leaf from the context it computes itself (the kernel's
measurement and boot epoch, or a capsule's measurement, capability word and
policy epoch), folds the trailer's path from that leaf, and compares with the
root compiled into it. `nonos-attest-path` is that fold. This module is its
specification, over the fold `Nonos.Stark.Merkle.recompute` already describes:
at each level the running node goes right of its sibling when the direction bit
is set, which is `verify.rs`'s `if path.right(k) { compress(&sib, &node) }`.

Two statements matter.

`accepted_leaf_is_in_the_tree`: if the fold reaches the root of a balanced tree
along a path of the tree's depth, the leaf it started from is the tree's leaf at
the position the directions name. The fold accepts only a leaf that is on a path
to the root.

`an_accepted_context_is_an_enrolled_one`: if the gate accepts a context, that
context is exactly the one an enrolled slot holds. The capsule context carries
the capability word, so a capsule presented under a capability word it was not
enrolled with is refused on every path. Before the context was in the leaf, the
leaf was the image alone and this was false: anyone could present an enrolled
capsule under any capability word.

Both carry their assumptions as named hypotheses rather than axioms, so the
release root is untouched. The compression is assumed pairwise injective, which
for the real Poseidon means collision resistance, as `Nonos.Stark.Merkle` states
it. The leaf function is assumed injective on slots: the Rust leaf puts the kind
in its own lane beside the digest of a length-prefixed context, so the encoding
is injective and the assumption is again only collision resistance of the hash. Neither can be
proved, and neither is hidden.

`an_opening_under_a_pinned_kind_is_that_kind` is the anonymous proof's case. The
circuit never sees a context, only a digest as a private witness and the kind as
a constant it pins. The v3 leaf puts the kind in its own lane of the permutation
for this reason: with the slot taken as the pair of kind and digest, an opening
that reaches the root under the kernel kind is a kernel slot. Under v2 the kind
was inside the digest, the circuit could not pin it, and a padding slot, whose
digest came from a public constant, opened as an approved kernel.
-/

import Nonos.Stark.Merkle

namespace Nonos.AttestPath

open Nonos.Stark.Merkle

/-- A policy tree: slots at the leaves, every node the compression of its two
    children. -/
inductive Tree (α : Type) where
  | leaf : α → Tree α
  | node : Tree α → Tree α → Tree α

/-- The root the enroll tool commits and the gates compare against. -/
def rootOf {α : Type} (f : α → α → α) : Tree α → α
  | .leaf a => a
  | .node l r => f (rootOf f l) (rootOf f r)

/-- Every leaf at depth `n`, as the enroll tool builds the tree: padded to `2^n`
    slots with the Pad kind. -/
inductive Balanced {α : Type} : Tree α → Nat → Prop
  | leaf (a : α) : Balanced (.leaf a) 0
  | node {l r : Tree α} {n : Nat} : Balanced l n → Balanced r n → Balanced (.node l r) (n + 1)

/-- The leaf reached by following `route` down from the root, `true` meaning
    right. -/
def leafAt {α : Type} : Tree α → List Bool → Option α
  | .leaf a, [] => some a
  | .node l _, false :: rest => leafAt l rest
  | .node _ r, true :: rest => leafAt r rest
  | _, _ => none

/-- The tree of leaves the gate folds over: each slot measured to its leaf. -/
def Tree.map {α β : Type} (g : α → β) : Tree α → Tree β
  | .leaf a => .leaf (g a)
  | .node l r => .node (l.map g) (r.map g)

/-- The last step of a path is the root-level combination. -/
theorem recompute_snoc {α : Type} (f : α → α → α) (s : Step α) :
    ∀ (steps : List (Step α)) (l : α),
      recompute f l (steps ++ [s]) =
        (if s.onRight then f s.sibling (recompute f l steps)
         else f (recompute f l steps) s.sibling)
  | [], l => by simp [recompute]
  | x :: rest, l => by
      simp only [List.cons_append, recompute]
      exact recompute_snoc f s rest _

/-- Soundness of the fold. `rsteps` is the path read from the root down, so the
    gate folds `rsteps.reverse`, leaf first. If that reaches the root of a
    balanced tree of the path's depth, the leaf is the tree's leaf at the
    position the direction bits name. -/
theorem accepted_leaf_is_in_the_tree {α : Type} (f : α → α → α)
    (hinj : ∀ a b a' b', f a b = f a' b' → a = a' ∧ b = b') :
    ∀ (rsteps : List (Step α)) (t : Tree α) (l : α),
      Balanced t rsteps.length →
      recompute f l rsteps.reverse = rootOf f t →
      leafAt t (rsteps.map (·.onRight)) = some l
  | [], _, l, hb, h => by
      cases hb with
      | leaf a =>
        simp [recompute, rootOf] at h
        subst h
        simp [leafAt]
  | s :: rest, _, l, hb, h => by
      cases hb with
      | node hl hr =>
        rw [List.reverse_cons, recompute_snoc f s] at h
        simp only [rootOf] at h
        cases hs : s.onRight with
        | true =>
          simp [hs] at h
          simp only [List.map_cons, hs, leafAt]
          exact accepted_leaf_is_in_the_tree f hinj rest _ l hr (hinj _ _ _ _ h).2
        | false =>
          simp [hs] at h
          simp only [List.map_cons, hs, leafAt]
          exact accepted_leaf_is_in_the_tree f hinj rest _ l hl (hinj _ _ _ _ h).1

theorem balanced_map {α β : Type} (g : α → β) :
    ∀ {t : Tree α} {n : Nat}, Balanced t n → Balanced (t.map g) n
  | _, _, .leaf a => .leaf (g a)
  | _, _, .node hl hr => .node (balanced_map g hl) (balanced_map g hr)

theorem leafAt_map {α β : Type} (g : α → β) :
    ∀ (t : Tree α) (r : List Bool), leafAt (t.map g) r = (leafAt t r).map g
  | .leaf _, [] => rfl
  | .leaf _, _ :: _ => rfl
  | .node _ _, [] => rfl
  | .node l _, false :: rest => by
      simp only [Tree.map, leafAt]
      exact leafAt_map g l rest
  | .node _ r, true :: rest => by
      simp only [Tree.map, leafAt]
      exact leafAt_map g r rest

/-- The gate: the leaf of the presented slot, folded up the presented path,
    compared with the root. -/
def accepts {α σ : Type} (f : α → α → α) (leafOf : σ → α) (slot : σ)
    (steps : List (Step α)) (root : α) : Prop :=
  recompute f (leafOf slot) steps = root

/-- An accepted context is an enrolled one: the gate accepts a slot only when the
    enrolled tree holds exactly that slot at the position the path names. A slot
    is a kind and a context, and the capsule context carries the capability word
    and the epoch, so this is the capability binding. -/
theorem an_accepted_context_is_an_enrolled_one {α σ : Type} (f : α → α → α)
    (hinj : ∀ a b a' b', f a b = f a' b' → a = a' ∧ b = b')
    (leafOf : σ → α) (hleaf : ∀ x y, leafOf x = leafOf y → x = y)
    (t : Tree σ) (rsteps : List (Step α)) (slot : σ)
    (hb : Balanced t rsteps.length)
    (h : accepts f leafOf slot rsteps.reverse (rootOf f (t.map leafOf))) :
    leafAt t (rsteps.map (·.onRight)) = some slot := by
  have hacc := accepted_leaf_is_in_the_tree f hinj rsteps (t.map leafOf) (leafOf slot)
    (balanced_map leafOf hb) h
  rw [leafAt_map] at hacc
  cases hl : leafAt t (rsteps.map (·.onRight)) with
  | none =>
    rw [hl] at hacc
    simp at hacc
  | some s =>
    rw [hl] at hacc
    simp at hacc
    rw [hleaf _ _ hacc]

/-- The case the capability gap was: a capsule presented under a context no slot
    holds (an enrolled image with capabilities it was not enrolled with) is
    refused, whatever path it brings. -/
theorem an_unenrolled_context_is_refused {α σ : Type} (f : α → α → α)
    (hinj : ∀ a b a' b', f a b = f a' b' → a = a' ∧ b = b')
    (leafOf : σ → α) (hleaf : ∀ x y, leafOf x = leafOf y → x = y)
    (t : Tree σ) (rsteps : List (Step α)) (slot : σ)
    (hb : Balanced t rsteps.length)
    (hnot : ∀ r, leafAt t r ≠ some slot) :
    ¬ accepts f leafOf slot rsteps.reverse (rootOf f (t.map leafOf)) := fun h =>
  hnot _ (an_accepted_context_is_an_enrolled_one f hinj leafOf hleaf t rsteps slot hb h)

/-- The anonymous proof's statement: a digest opened under a pinned kind reaches
    the root only from a slot that holds that kind and that digest. Under v2 the
    kind was inside the digest and could not be pinned, so this had no analogue:
    a padding digest opened as a kernel. -/
theorem an_opening_under_a_pinned_kind_is_that_kind {α κ δ : Type} (f : α → α → α)
    (hinj : ∀ a b a' b', f a b = f a' b' → a = a' ∧ b = b')
    (leafOf : κ × δ → α) (hleaf : ∀ x y, leafOf x = leafOf y → x = y)
    (t : Tree (κ × δ)) (rsteps : List (Step α)) (kind : κ) (d : δ)
    (hb : Balanced t rsteps.length)
    (h : accepts f leafOf (kind, d) rsteps.reverse (rootOf f (t.map leafOf))) :
    ∃ r, leafAt t r = some (kind, d) :=
  ⟨_, an_accepted_context_is_an_enrolled_one f hinj leafOf hleaf t rsteps (kind, d) hb h⟩

end Nonos.AttestPath
