/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

The resolver, on the extracted code.

`Refinement.lean` proves the single-capability operations compute the `capsOf`
denotation. All three take a capability the caller already named, so none of them
can see the defect this file is about: a capability that is granted in a token
word and then absent from the list the kernel builds from that word. It resolves
to nothing, and nothing anywhere fails. `ForeignExec` was in that state for a
release and no Linux binary ran.

`select_caps` is where that happens, and until now it could not be proven about at
all: it was written with iterator adapters, which are outside the fragment Aeneas
translates, so the extraction stopped short of the one function the bug lived in.
It is a loop now, for that reason, and the theorems below are about the extracted
loop rather than about a model of it.

`select_caps_spec` is the whole content: the resolved list is exactly the
capabilities of the table whose bit is set, in table order.
`granting_resolves` is the corollary that would have failed: granting a
capability and reading the word back yields that capability, for every capability
in the table. `resolving_is_sound` is the other direction, and it is the half that
matters for confinement: nothing resolves out of a word that the word does not
grant.

Table completeness is a separate claim and it is not proven here. `Capability::all`
is a static, so Charon leaves it opaque; what makes it complete is that the
`capability_table!` macro generates the enumeration and the list from one source,
which is a structural argument about the macro rather than a theorem about
extracted code. These theorems say the resolver is faithful to the table it is
handed. That the table is the whole enumeration is the macro's job, and
`guard.rs` checks the rest at compile time.

The axiom profile is at the end of the file rather than in a separate ledger, so
the CI step that builds this library prints it: standard axioms only, and not even
Aeneas's opaque `Option::ok_or`, because nothing in the closure of these
definitions calls it.
-/

import NonosExtraction.Caps
import NonosExtraction.Refinement
import Nonos.Capability
import Nonos.CapabilityBits

open Aeneas Aeneas.Std Result
open Nonos.Capability (Grants)
open Nonos.CapabilityBits (capsOf)
open nonos_caps

set_option linter.hashCommand false

namespace NonosExtraction

/-- The extracted enumeration, abbreviated. -/
abbrev Cap := capabilities.types.defs.Capability

/-! ### What the resolver is supposed to return -/

/-- The capabilities of a table that a token word grants, in table order. This is
    the specification; everything below connects the extracted loop to it. -/
def selected (table : List Cap) (bits : Nat) : List Cap :=
  table.filter (fun c => capsOf bits (idx c))

/-- Nothing is selected out of an empty table. -/
@[simp]
theorem selected_nil (bits : Nat) : selected [] bits = [] := rfl

/-- One step of the specification, matching one turn of the loop. -/
theorem selected_cons (c : Cap) (rest : List Cap) (bits : Nat) :
    selected (c :: rest) bits =
      (if capsOf bits (idx c) then [c] else []) ++ selected rest bits := by
  unfold selected
  rw [List.filter_cons]
  cases h : capsOf bits (idx c) <;> simp

/-- Every selected capability came from the table. -/
theorem selected_subset (table : List Cap) (bits : Nat) (c : Cap)
    (h : c ∈ selected table bits) : c ∈ table := by
  unfold selected at h
  exact List.mem_of_mem_filter h

/-- And its bit is set, which is the soundness half stated on the
    specification. -/
theorem selected_granted (table : List Cap) (bits : Nat) (c : Cap)
    (h : c ∈ selected table bits) : capsOf bits (idx c) = true := by
  unfold selected at h
  have := List.of_mem_filter h
  simpa using this

/-- Membership in the selection is exactly membership in the table together with
    the bit being set. -/
theorem mem_selected (table : List Cap) (bits : Nat) (c : Cap) :
    c ∈ selected table bits ↔ c ∈ table ∧ capsOf bits (idx c) = true := by
  simp [selected, List.mem_filter]

/-- One more element of the table, on the specification side. Walking one step
    appends the selection of the element just reached. -/
theorem selected_take_succ (table : List Cap) (bits i n : Nat)
    (hin : i ≤ n) (hn : n < table.length) :
    selected ((table.take (n + 1)).drop i) bits =
      selected ((table.take n).drop i) bits ++
        (if capsOf bits (idx table[n]) then [table[n]] else []) := by
  have htake : table.take (n + 1) = table.take n ++ [table[n]] := by
    rw [List.take_succ]
    congr 1
    simp [List.getElem?_eq_getElem hn]
  have hlen : (table.take n).length = n := by
    rw [List.length_take]
    omega
  rw [htake, List.drop_append_of_le_length (by omega)]
  unfold selected
  rw [List.filter_append]
  congr 1
  rw [List.filter_cons]
  cases h : capsOf bits (idx table[n]) <;> simp

/-! ### The extracted loop -/

private theorem u64_ne_zero' (x : Std.U64) : (x != 0#u64) = decide (x.val ≠ 0) := by
  simp [bne, BEq.beq, BitVec.toNat_eq]

private theorem nat_bridge' (n k : Nat) : (decide (n &&& 2 ^ k ≠ 0)) = n.testBit k := by
  rw [Nat.and_two_pow]
  cases h : n.testBit k <;> simp

/-- The bit table as a step lemma, so the tactic that walks the loop body can get
    past the call. `Refinement.bit_spec` is the content; this is the same fact in
    the form the walker consumes. -/
@[local step]
theorem bit_step (cap : Cap) :
    capabilities.types.defs.Capability.bit cap ⦃ v => v.val = 2 ^ idx cap ⦄ := by
  obtain ⟨v, hv, hval⟩ := bit_spec cap
  rw [hv]
  first
  | simp [hval]
  | (simp only [WP.spec, WP.theta]; simp [hval])
  | grind [hval]


/-- The loop accumulates the selection of what it has not yet walked.

    The invariant is stated over `drop`: whatever is already in `out` stays, and
    what the loop adds is the selection of the rest of the table. The bound on the
    length is what `Vec.push` needs, and it holds at the call site because the
    output is never longer than the table. -/
@[step]
theorem select_caps_loop_spec (table : Slice Cap) (bits : Std.U64)
    (out : alloc.vec.Vec Cap) (i : Std.Usize)
    (hi : i.val ≤ table.length)
    (hlen : out.val.length + (table.length - i.val) ≤ Usize.max) :
    capabilities.bits.select_caps_loop table bits out i ⦃ out' =>
      out'.val = out.val ++ selected (table.val.drop i.val) bits.val ⦄ := by
  unfold capabilities.bits.select_caps_loop
  apply loop.spec_decr_nat
    (measure := fun (st : (alloc.vec.Vec Cap) × Std.Usize) => table.length - st.2.val)
    (inv := fun (st : (alloc.vec.Vec Cap) × Std.Usize) =>
      i.val ≤ st.2.val ∧
      st.2.val ≤ table.length ∧
      st.1.val.length + (table.length - st.2.val) ≤ Usize.max ∧
      st.1.val = out.val ++ selected (table.val.take st.2.val |>.drop i.val) bits.val)
  · intro st hst
    obtain ⟨hlo, hle, hmax, heq⟩ := hst
    unfold capabilities.bits.select_caps_loop.body
    step*
    case _ =>
      /-
       * The walking case. The condition on the token word is the denotation, the
       * push appends the element just reached, and the specification for one more
       * element of the table appends exactly the same thing.
       -/
      have hlt : st.2.val < table.length := by scalar_tac
      have hcond : (i3 != 0#u64) = capsOf bits.val (idx cap) := by
        rw [u64_ne_zero']
        have hand : i3.val = bits.val &&& 2 ^ idx cap := by
          rw [i3_post1]
          simp [UScalar.val_and, i2_post]
        rw [hand, nat_bridge']
        simp [Nonos.CapabilityBits.capsOf]
      rw [hcond]
      split
      · step as ⟨out1, hout1⟩
        step as ⟨i4, hi4⟩
        have hi4' : i4.val = st.2.val + 1 := by scalar_tac
        have hout1' : out1.val.length = st.1.val.length + 1 := by
          rw [hout1]; simp
        refine ⟨by omega, by omega, by omega, ?_, by omega⟩
        rw [hout1, heq, hi4']
        rw [selected_take_succ table.val bits.val i.val st.2.val (by scalar_tac)
          (by scalar_tac)]
        rw [cap_post] at *
        simp_all
      · step as ⟨i4, hi4⟩
        have hi4' : i4.val = st.2.val + 1 := by scalar_tac
        refine ⟨by omega, by omega, by omega, ?_, by omega⟩
        rw [heq, hi4']
        rw [selected_take_succ table.val bits.val i.val st.2.val (by scalar_tac)
          (by scalar_tac)]
        rw [cap_post] at *
        simp_all
    case _ =>
      /- The exit case: the index has reached the end, so the walked prefix is the
         whole table. -/
      have hend : st.2.val = table.val.length := by scalar_tac
      rw [heq, hend]
      simp
  · exact ⟨Nat.le_refl _, hi, hlen, by simp⟩

/-- The resolver returns exactly the capabilities of its table that the word
    grants.

    This is the theorem the resolver could not carry before: it is about the
    extracted definition, which is the real `select_caps` lowered from the MIR
    rustc compiles. -/
theorem select_caps_spec (table : Slice Cap) (bits : Std.U64) :
    capabilities.bits.select_caps table bits ⦃ out =>
      out.val = selected table.val bits.val ⦄ := by
  unfold capabilities.bits.select_caps
  have hmax : (alloc.vec.Vec.new Cap).val.length + (table.length - 0) ≤ Usize.max := by
    simp [Slice.length_ineq table]
  have := select_caps_loop_spec table bits (alloc.vec.Vec.new Cap) 0#usize (by simp) hmax
  simpa using this

/-! ### What that buys -/

/-- The specification in the form the corollaries below use: the call succeeds and
    the list it returns is the selection. -/
theorem select_caps_eq (table : Slice Cap) (bits : Std.U64) :
    ∃ out, capabilities.bits.select_caps table bits = ok out ∧
      out.val = selected table.val bits.val :=
  WP.spec_imp_exists (select_caps_spec table bits)

/-- Granting a capability and reading the word back yields that capability, for
    every capability the table lists.

    False exactly when the table omits the capability, which is the shape of the
    `ForeignExec` defect. It is a theorem about the function the kernel calls now
    rather than about a list written out by hand. -/
theorem granting_resolves (table : Slice Cap) (c : Cap) (hmem : c ∈ table.val)
    (bits : Std.U64) (hgrant : capsOf bits.val (idx c) = true) :
    ∃ out, capabilities.bits.select_caps table bits = ok out ∧ c ∈ out.val := by
  obtain ⟨out, hcall, hout⟩ := select_caps_eq table bits
  refine ⟨out, hcall, ?_⟩
  rw [hout]
  exact (mem_selected table.val bits.val c).mpr ⟨hmem, hgrant⟩

/-- Nothing resolves out of a word that does not grant it. The confinement half:
    a capsule's resolved list never names authority its token lacks. -/
theorem resolving_is_sound (table : Slice Cap) (bits : Std.U64) :
    ∃ out, capabilities.bits.select_caps table bits = ok out ∧
      ∀ c ∈ out.val, capsOf bits.val (idx c) = true := by
  obtain ⟨out, hcall, hout⟩ := select_caps_eq table bits
  refine ⟨out, hcall, ?_⟩
  intro c hc
  rw [hout] at hc
  exact selected_granted table.val bits.val c hc

/-- And nothing resolves that was not in the table, so the resolver cannot invent
    a capability the table does not define. -/
theorem resolving_stays_in_the_table (table : Slice Cap) (bits : Std.U64) :
    ∃ out, capabilities.bits.select_caps table bits = ok out ∧
      ∀ c ∈ out.val, c ∈ table.val := by
  obtain ⟨out, hcall, hout⟩ := select_caps_eq table bits
  refine ⟨out, hcall, ?_⟩
  intro c hc
  rw [hout] at hc
  exact selected_subset table.val bits.val c hc

/-- Resolution is exact in both directions at once: the list the kernel builds
    from a token word names exactly the capabilities of the table that word
    grants. -/
theorem resolution_is_faithful (table : Slice Cap) (bits : Std.U64) (c : Cap) :
    ∃ out, capabilities.bits.select_caps table bits = ok out ∧
      (c ∈ out.val ↔ c ∈ table.val ∧ capsOf bits.val (idx c) = true) := by
  obtain ⟨out, hcall, hout⟩ := select_caps_eq table bits
  refine ⟨out, hcall, ?_⟩
  rw [hout]
  exact mem_selected table.val bits.val c

/-- An empty token resolves to nothing: a guest spawned with no capabilities holds
    none, over the extracted resolver rather than over the spawn path's comment. -/
theorem empty_token_resolves_to_nothing (table : Slice Cap) :
    ∃ out, capabilities.bits.select_caps table 0#u64 = ok out ∧ out.val = [] := by
  obtain ⟨out, hcall, hout⟩ := select_caps_eq table 0#u64
  refine ⟨out, hcall, ?_⟩
  rw [hout]
  unfold selected
  refine List.filter_eq_nil_iff.mpr ?_
  intro c _
  simp [Nonos.CapabilityBits.capsOf]

end NonosExtraction

/-! ### Axiom profile

    Printed by the build. `sorryAx` in any of these lines would mean a `sorry`
    reached the closure of a theorem about extracted code. -/

#print axioms NonosExtraction.selected_take_succ
#print axioms NonosExtraction.select_caps_loop_spec
#print axioms NonosExtraction.select_caps_spec
#print axioms NonosExtraction.select_caps_eq
#print axioms NonosExtraction.granting_resolves
#print axioms NonosExtraction.resolving_is_sound
#print axioms NonosExtraction.resolving_stays_in_the_table
#print axioms NonosExtraction.resolution_is_faithful
#print axioms NonosExtraction.empty_token_resolves_to_nothing
#print axioms NonosExtraction.mem_selected
#print axioms NonosExtraction.selected_granted
#print axioms NonosExtraction.selected_subset
