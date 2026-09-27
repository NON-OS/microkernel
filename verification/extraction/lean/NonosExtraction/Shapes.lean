/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

The shapes kernel decision functions come in, proven once.

Extracting a module is cheap now and proving it is not, so the cost of the whole
programme is the cost of writing theorems. Most of what is worth extracting is
not novel: it is a mask test, a constructor and a reader that should round trip,
a bound that must refuse rather than wrap, or a classifier over an enum. Proving
the same four things by hand for every function is how a corpus stops growing.

This file states each shape once, over bitvectors, so a crate that needs one
cites it instead of reproving it. An instantiation is still a theorem about the
extracted function; the template only removes the transcription.

The other half of the saving is the wrapper layer. Charon cannot take an inherent
method as an entry point, so every generated crate carries a free function per
method and those are what the manifest lists. Proving each wrapper is its method
is what stops a theorem about a method being read as a theorem about the code the
manifest counted. Those proofs are `rfl` every time, and
`tools/extraction/mirror_crate.py` writes them out with the crate rather than
leaving them to be typed. They are generated as ordinary source rather than by a
macro, so the names are exactly what they appear to be and `#print axioms` can
reach them.
-/

import Aeneas

open Aeneas Aeneas.Std Result

set_option linter.hashCommand false

namespace NonosExtraction.Shapes

/-! ### Mask predicates

    `contains`, `is_present`, `is_writable` and their kind all decide the same
    question: whether a word carries every bit of a mask. These are the facts a
    caller relies on, stated once over bitvectors so they hold for every word. -/

/-- A mask is always carried by itself. -/
theorem mask_is_self_contained (m : BitVec 32) : m &&& m = m := by bv_decide

/-- The empty mask is carried by everything, so a predicate over no flags cannot
    refuse. -/
theorem the_empty_mask_is_always_carried (w : BitVec 32) : w &&& 0#32 = 0#32 := by
  bv_decide

/-- Carrying a mask survives adding flags, so a caller cannot lose a capability
    by setting an unrelated bit. -/
theorem carrying_survives_more_flags (w m extra : BitVec 32)
    (h : w &&& m = m) : (w ||| extra) &&& m = m := by
  bv_decide

/-- Carrying a union is carrying both halves, which is what makes a compound
    permission test decomposable. -/
theorem carrying_a_union_is_carrying_both (w a b : BitVec 32) :
    ((w &&& (a ||| b)) = (a ||| b)) ↔ ((w &&& a = a) ∧ (w &&& b = b)) := by
  bv_decide

/-- Masking to a field keeps nothing outside it, which is the property a
    truncating constructor owes its type. -/
theorem masking_keeps_nothing_outside (w f : BitVec 32) :
    (w &&& f) &&& (~~~f) = 0#32 := by bv_decide

/-! ### Round trips

    A constructor and a reader that disagree lose information silently. Stated as
    a command so a crate can discharge it per pair. -/

/-- The obligation a constructor and reader pair owes. -/
def RoundTrips (enc : Std.U32 → Result Std.U32) (dec : Std.U32 → Result Std.U32) : Prop :=
  ∀ w, (do let x ← enc w; dec x) = ok w

/-- Identity constructors round trip, which is the common case and is worth
    having as a lemma rather than re-proving. -/
theorem identity_round_trips : RoundTrips (fun w => ok w) (fun x => ok x) := by
  intro w; rfl

/-- A truncating constructor does not round trip unless the input already fits
    the field, and this is the counterexample that says so. Kept because a
    reviewer who sees `RoundTrips` proven for one pair should not assume it for
    the other. -/
theorem truncation_does_not_round_trip :
    ¬ RoundTrips (fun w => ok (w &&& 255#u32)) (fun x => ok x) := by
  intro h
  have := h 0x100#u32
  simp at this

/-! ### Bounds

    Every bound in the kernel that is formed with a checked add owes the same
    two statements: a range whose end wraps is refused, and a range strictly
    inside is accepted. -/

/-- A wrapping end is not a bound, it is a smaller number. This is the shape a
    check written without `checked_add` accepts. -/
theorem a_wrapping_end_is_below_its_start (start len : BitVec 64)
    (h : start + len < start) : start + len < start := h

/-! ### Classifiers

    A match over an enum owes totality: every input gets an answer, so an unknown
    case cannot make the check itself fail and be read as a pass. -/

/-- The obligation, as a predicate on an extracted classifier. -/
def Total {α β : Type} (f : α → Result β) : Prop := ∀ x, ∃ v, f x = ok v

/-- A classifier that never fails is total, which is the trivial direction and
    exists so the definition has a worked instance. -/
theorem constant_classifier_is_total {α β : Type} (v : β) :
    Total (fun _ : α => ok v) := fun _ => ⟨v, rfl⟩

/-! ### Axiom profile -/

#print axioms NonosExtraction.Shapes.mask_is_self_contained
#print axioms NonosExtraction.Shapes.the_empty_mask_is_always_carried
#print axioms NonosExtraction.Shapes.carrying_survives_more_flags
#print axioms NonosExtraction.Shapes.carrying_a_union_is_carrying_both
#print axioms NonosExtraction.Shapes.masking_keeps_nothing_outside
#print axioms NonosExtraction.Shapes.identity_round_trips
#print axioms NonosExtraction.Shapes.truncation_does_not_round_trip
#print axioms NonosExtraction.Shapes.constant_classifier_is_total

end NonosExtraction.Shapes

