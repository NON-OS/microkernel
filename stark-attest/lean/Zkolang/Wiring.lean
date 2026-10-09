/-
 zKølang by NØNOS
 AGPL-3.0-or-later
-/

/-!
The wiring argument that replaces one running product per binding with a single
permutation over the whole trace.

A binding is a class of trace cells forced equal, and a class is enforced by
rotating it into a cycle of the permutation. Thirteen hundred separate products
were accidentally safe: each owned its cycles, so no binding could disturb
another. One permutation removes that, and a class laid over a cell that already
carries an image rewrites the earlier cycle and drops the earlier binding while
every other binding still holds and every aggregate test still passes.

Disjointness is what stands in for the lost ownership, so it is the precondition
the whole collapse rests on. Tests sample; the claim is universal, so it is
proven here instead: a cell outside a class keeps its image, so laying down a
disjoint class leaves an earlier one exactly as it was.
-/

namespace Zkolang.Wiring

/-- Position of a cell in a class, if the class holds it. -/
def indexOf : List Nat → Nat → Option Nat
  | [], _ => none
  | c :: cs, x => if c = x then some 0 else (indexOf cs x).map (· + 1)

/-- The successor of a cell in its class, cyclically. A cell the class does not
hold is its own successor, which is what keeps disjoint classes apart. -/
def next (c : List Nat) (x : Nat) : Nat :=
  match indexOf c x with
  | none => x
  | some i => c.getD ((i + 1) % c.length) x

theorem indexOf_none_of_not_mem : ∀ (c : List Nat) (x : Nat), x ∉ c → indexOf c x = none := by
  intro c
  induction c with
  | nil => intro x _; rfl
  | cons a as ih =>
    intro x hx
    have hne : a ≠ x := by
      intro h; exact hx (by rw [h]; exact List.mem_cons_self x as)
    have htl : x ∉ as := fun h => hx (List.mem_cons_of_mem a h)
    simp [indexOf, hne, ih x htl]

/-- A cell no class holds keeps its image. -/
theorem next_of_not_mem (c : List Nat) (x : Nat) (h : x ∉ c) : next c x = x := by
  simp [next, indexOf_none_of_not_mem c x h]

/-- Laying down a class leaves every cell outside it untouched, so a class laid
over disjoint cells cannot move what an earlier class arranged. This is the
property that replaces per binding ownership. -/
theorem disjoint_class_preserves
    (c d : List Nat) (x : Nat) (hx : x ∈ c) (hcd : ∀ y, y ∈ c → y ∉ d) :
    next d x = x :=
  next_of_not_mem d x (hcd x hx)

end Zkolang.Wiring
