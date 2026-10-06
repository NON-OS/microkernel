/-
 zKølang by NØNOS
 AGPL-3.0-or-later
-/

/-!
The opening boundary in the batched Merkle membership AIR.

Each opening's first state is the leaf injected against its first sibling, in the
order the path bit gives. The per-proof form pins that state from a periodic
column, which bakes the opening's own leaf into the columns a verifier key
commits to and moves the key with the witness. The production form leaves the
state to the witness instead.

What makes that safe is the copy constraint the assembly already places on the
opened cell: it pins the half of the state the leaf occupies, chosen by the same
path bit. So the prover's remaining freedom is the sibling, which is path data
the committed root pins by preimage resistance rather than by an AIR constraint.

Proven here: a state whose node half is the bound leaf *is* an injection of that
leaf. Pinning the half is not weaker than constructing the whole. Tests sample
the states a prover happens to produce; the claim is over all of them.
-/

namespace Zkolang.Opening

variable {F : Type}

/-- Compression input: the running node and its sibling, ordered by the path bit.
`false` puts the node in the low half. -/
def inject (node sib : List F) : Bool → List F
  | false => node ++ sib
  | true => sib ++ node

/-- The half of a state the node occupies, selected by the same path bit. This is
the cell `opened_cells` hands to the wiring engine. -/
def nodeHalf (s : List F) (rate : Nat) : Bool → List F
  | false => s.take rate
  | true => s.drop rate

/-- A state whose node half is the bound leaf is an injection of that leaf. The
witness keeps only the sibling, so dropping the reset constraint does not widen
what a prover can put into the walk. -/
theorem injected_of_nodeHalf (s leaf : List F) (rate : Nat) :
    ∀ dir, nodeHalf s rate dir = leaf → ∃ sib, s = inject leaf sib dir
  | false, h => ⟨s.drop rate, by
      rw [inject, ← h, nodeHalf, List.take_append_drop]⟩
  | true, h => ⟨s.take rate, by
      rw [inject, ← h, nodeHalf, List.take_append_drop]⟩

/-- And the pin reads back what was injected, so the constraint the assembly
places is the one this argument assumes. -/
theorem nodeHalf_inject (leaf sib : List F) (rate : Nat)
    (hl : leaf.length = rate) (hs : sib.length = rate) :
    ∀ dir, nodeHalf (inject leaf sib dir) rate dir = leaf
  | false => by
      rw [inject, nodeHalf, ← hl, List.take_left]
  | true => by
      rw [inject, nodeHalf, ← hs, List.drop_left]

end Zkolang.Opening
