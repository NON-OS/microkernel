/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

The folder, on the extracted code.

`fold_caps` turns a table of capabilities into a token word: it ORs each entry's
bit into an accumulator. `caps_to_bits` is `fold_caps` from zero, and it is how a
capsule's declared capabilities become the word the kernel checks, so a table
entry whose bit went missing here would be a capability the capsule asked for and
never held, and an entry that set a bit it does not own would be authority nobody
granted.

`fold_caps_spec` is the whole content: the extracted loop computes the fold of
the table's bits into the starting word. `folding_grants_exactly` is what a caller
relies on: the word it returns grants a capability exactly when the starting word
already granted it or the table lists it, and nothing else. `folding_then_resolving`
closes the loop with the resolver in `CapsComplete`: fold a table from the empty
word and resolve the result against the same table, and every entry comes back.

What these theorems do not say: that a table is the whole enumeration. The table is
an argument, and `Capability::all` is a static Charon leaves opaque; that it is
complete is the `capability_table!` macro's job, as `CapsComplete` states.
-/

import NonosExtraction.Caps
import NonosExtraction.Refinement
import NonosExtraction.CapsComplete
import Nonos.CapabilityBits

open Aeneas Aeneas.Std Result
open Nonos.CapabilityBits (capsOf)
open nonos_caps

set_option linter.hashCommand false

namespace NonosExtraction

/-! ### Distinct capabilities own distinct bits -/

/-- The capability at a bit index, the inverse of `idx`. -/
def ofIdx : Nat → Option Cap
  | 0 => some .CoreExec
  | 1 => some .IO
  | 2 => some .Network
  | 3 => some .IPC
  | 4 => some .Memory
  | 5 => some .Crypto
  | 6 => some .FileSystem
  | 7 => some .Hardware
  | 8 => some .Debug
  | 9 => some .Admin
  | 10 => some .RegisterService
  | 11 => some .GraphicsDisplayQuery
  | 12 => some .GraphicsSurfaceCreate
  | 13 => some .GraphicsSurfaceMap
  | 14 => some .GraphicsPresent
  | 15 => some .DeviceEnum
  | 16 => some .Driver
  | 17 => some .Mmio
  | 18 => some .Irq
  | 19 => some .Dma
  | 20 => some .Pio
  | 21 => some .InputSource
  | 22 => some .TimeSet
  | 23 => some .SpawnBroker
  | 24 => some .SpawnWindow
  | 25 => some .ProcessControl
  | 26 => some .StoreWrite
  | 27 => some .EnrolDevRoot
  | 28 => some .Keyring
  | 29 => some .Entropy
  | 30 => some .AppInstall
  | 31 => some .AttestRead
  | 32 => some .ForeignExec
  | 33 => some .LocalSign
  | _ => none

theorem ofIdx_idx (c : Cap) : ofIdx (idx c) = some c := by
  cases c <;> rfl

/-- No two capabilities share a bit. Without this a word could not tell two of
    them apart, and granting one would grant the other. -/
theorem idx_inj {a b : Cap} (h : idx a = idx b) : a = b := by
  have ha := ofIdx_idx a
  rw [h, ofIdx_idx] at ha
  exact (Option.some.inj ha).symm

/-! ### What the folder is supposed to return -/

/-- The table's bits ORed into a starting word, in table order. This is the
    specification; the extracted loop is connected to it below. -/
def folded (table : List Cap) (acc : Nat) : Nat :=
  table.foldl (fun a c => a ||| 2 ^ idx c) acc

theorem folded_append_single (l : List Cap) (c : Cap) (acc : Nat) :
    folded (l ++ [c]) acc = folded l acc ||| 2 ^ idx c := by
  simp [folded, List.foldl_append]

/-- A bit of the fold is set exactly when the starting word had it or some entry
    of the table owns it. -/
theorem folded_testBit (table : List Cap) (acc k : Nat) :
    (folded table acc).testBit k = (acc.testBit k || table.any (fun c => idx c == k)) := by
  induction table generalizing acc with
  | nil => simp [folded]
  | cons c rest ih =>
    have hstep : folded (c :: rest) acc = folded rest (acc ||| 2 ^ idx c) := rfl
    rw [hstep, ih, Nat.testBit_or, Nat.testBit_two_pow, List.any_cons, Bool.or_assoc]
    congr 2

/-- In the form the grant semantics reads: the fold grants a capability exactly
    when the starting word does or the table lists it, and grants nothing else. -/
theorem folded_grants (table : List Cap) (acc : Nat) (c : Cap) :
    capsOf (folded table acc) (idx c) = true ↔
      capsOf acc (idx c) = true ∨ c ∈ table := by
  unfold capsOf
  rw [folded_testBit, Bool.or_eq_true, List.any_eq_true]
  constructor
  · rintro (h | ⟨d, hd, hdc⟩)
    · exact Or.inl h
    · have hdc' : d = c := idx_inj (beq_iff_eq.mp hdc)
      subst hdc'
      exact Or.inr hd
  · rintro (h | h)
    · exact Or.inl h
    · exact Or.inr ⟨c, h, beq_self_eq_true _⟩

/-- One more element of the table on the specification side. -/
theorem folded_take_succ (table : List Cap) (acc i n : Nat)
    (hin : i ≤ n) (hn : n < table.length) :
    folded ((table.take (n + 1)).drop i) acc =
      folded ((table.take n).drop i) acc ||| 2 ^ idx table[n] := by
  have htake : table.take (n + 1) = table.take n ++ [table[n]] := by
    rw [List.take_add_one]
    congr 1
    simp [List.getElem?_eq_getElem hn]
  have hlen : (table.take n).length = n := by
    rw [List.length_take]
    omega
  rw [htake, List.drop_append_of_le_length (by omega), folded_append_single]

/-! ### The extracted loop -/

@[local step]
theorem bit_step_fold (cap : Cap) :
    capabilities.types.defs.Capability.bit cap ⦃ v => v.val = 2 ^ idx cap ⦄ := by
  obtain ⟨v, hv, hval⟩ := bit_spec cap
  rw [hv]
  simp [hval]

/-- The loop ORs in the bits of what it has not yet walked. -/
@[step]
theorem fold_caps_loop_spec (table : Slice Cap) (acc : Std.U64) (i : Std.Usize)
    (hi : i.val ≤ table.length) :
    capabilities.bits.fold_caps_loop table acc i ⦃ r =>
      r.val = folded (table.val.drop i.val) acc.val ⦄ := by
  unfold capabilities.bits.fold_caps_loop
  apply loop.spec_decr_nat
    (measure := fun (st : Std.U64 × Std.Usize) => table.length - st.2.val)
    (inv := fun (st : Std.U64 × Std.Usize) =>
      i.val ≤ st.2.val ∧
      st.2.val ≤ table.length ∧
      st.1.val = folded ((table.val.take st.2.val).drop i.val) acc.val)
  · intro st hst
    obtain ⟨hlo, hle, heq⟩ := hst
    unfold capabilities.bits.fold_caps_loop.body
    step*
    case _ =>
      have hlt : st.2.val < table.length := by scalar_tac
      refine ⟨by scalar_tac, by scalar_tac, ?_, by scalar_tac⟩
      /-
       * The walking case. The accumulator gains the bit of the entry just
       * reached, and the specification for one more entry of the table ORs in
       * exactly the same bit.
       -/
      rw [i3_post, folded_take_succ table.val acc.val i.val st.2.val hlo (by scalar_tac),
        ← heq, acc1_post1, UScalar.val_or, i2_post, c_post]
    case _ =>
      have hend : st.2.val = table.val.length := by scalar_tac
      rw [heq, hend]
      simp
  · exact ⟨Nat.le_refl _, hi, by simp [folded]⟩

/-- The folder returns the table's bits ORed into the starting word.

    About the extracted definition, which is the real `fold_caps` lowered from the
    MIR rustc compiles, not a model of it. -/
theorem fold_caps_spec (table : Slice Cap) (bits : Std.U64) :
    capabilities.bits.fold_caps table bits ⦃ r => r.val = folded table.val bits.val ⦄ := by
  unfold capabilities.bits.fold_caps
  have := fold_caps_loop_spec table bits 0#usize (by simp)
  simpa using this

/-! ### What that buys -/

/-- The specification in the form the corollaries use: the call succeeds and the
    word it returns is the fold. -/
theorem fold_caps_eq (table : Slice Cap) (bits : Std.U64) :
    ∃ r, capabilities.bits.fold_caps table bits = ok r ∧ r.val = folded table.val bits.val :=
  WP.spec_imp_exists (fold_caps_spec table bits)

/-- The word the folder returns grants a capability exactly when the starting word
    already granted it or the table lists it.

    Both halves matter. The forward half is that every capability a capsule
    declares ends up in its word. The backward half is confinement: folding a
    table never grants a capability that neither the table nor the starting word
    names. A bit table that gave two capabilities one bit would break the second
    half, and a loop that skipped an entry would break the first. -/
theorem folding_grants_exactly (table : Slice Cap) (bits : Std.U64) :
    ∃ r, capabilities.bits.fold_caps table bits = ok r ∧
      ∀ c : Cap, capsOf r.val (idx c) = true ↔
        capsOf bits.val (idx c) = true ∨ c ∈ table.val := by
  obtain ⟨r, hcall, hr⟩ := fold_caps_eq table bits
  refine ⟨r, hcall, fun c => ?_⟩
  rw [hr]
  exact folded_grants table.val bits.val c

/-- Fold a table from the empty word, resolve the word against the same table, and
    every entry comes back in order. This is `caps_to_bits` followed by the
    resolver, both on extracted code: a capability a capsule declares is one it
    holds when the kernel reads its token back. -/
theorem folding_then_resolving (table : Slice Cap) :
    ∃ r, capabilities.bits.fold_caps table 0#u64 = ok r ∧
      ∃ out, capabilities.bits.select_caps table r = ok out ∧ out.val = table.val := by
  obtain ⟨r, hcall, hr⟩ := fold_caps_eq table 0#u64
  obtain ⟨out, hsel, hout⟩ := select_caps_eq table r
  refine ⟨r, hcall, out, hsel, ?_⟩
  rw [hout]
  unfold selected
  refine List.filter_eq_self.mpr ?_
  intro c hc
  rw [hr]
  exact (folded_grants table.val (0#u64).val c).mpr (Or.inr hc)

end NonosExtraction

/-! ### Axiom profile

    Printed by the build. `sorryAx` in any of these lines would mean a `sorry`
    reached the closure of a theorem about extracted code. -/

#print axioms NonosExtraction.idx_inj
#print axioms NonosExtraction.folded_grants
#print axioms NonosExtraction.fold_caps_loop_spec
#print axioms NonosExtraction.fold_caps_spec
#print axioms NonosExtraction.folding_grants_exactly
#print axioms NonosExtraction.folding_then_resolving
