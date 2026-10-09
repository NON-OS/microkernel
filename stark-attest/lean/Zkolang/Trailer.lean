/-
 stark-attest by NØNOS
 AGPL-3.0-or-later
-/

/-!
The trailer parse is total: no input can make it read out of bounds.

`air/attest_trailer.rs::verify_attestation_trailer` is the first code a
verifier runs on bytes an attacker controls entirely. It slices a magic, a
depth byte, a direction bitfield and `depth * RATE` field elements out of a
`blob` whose length it does not choose. Rust would panic on a bad slice, and a
panic in a verifier is a denial of service at best and a parser differential at
worst, so the guards have to admit exactly the blobs whose reads are in range.
The Rust proves this by construction and by fuzzing; this module proves it
arithmetically, for every depth and every length.

The guards, in the order the Rust applies them:

  1. `blob.len() >= 9` and the magic matches, else reject
  2. `depth = blob[8]`, rejected when zero, so `1 <= depth <= 255`
  3. `dir_bytes = ceil(depth / 8)`
  4. `header = 9 + depth * RATE * 8 + dir_bytes`
  5. `blob.len() >= header`, else reject

The reads that follow are `blob[9 .. 9 + depth * RATE * 8]` for the siblings,
then `blob[9 + depth * RATE * 8 .. header]` for the direction bits, the order
the builder emits and every deployed consumer parses. Within the sibling
region, for each `level < depth` and `c < RATE`, the eight bytes at offset
`(level * RATE + c) * 8`. The theorems below show each of those lies inside the
length the guard established. This module once modelled the two regions in
the swapped order, faithfully mirroring a parser that disagreed with the
emitter beside it; the totality theorems held for that layout too, which is
the reminder that a proof is about the model it states, and the round-trip
test in the crate is what ties the model to the wire.

`RATE` is 4, the Poseidon rate the digests are packed at. It appears as a
literal here rather than an opaque constant so `omega` can close the goals; the
statements are specialised to the deployed parameter, which is what the
verifier actually runs.
-/

namespace Zkolang.Trailer

/-- The Poseidon rate: four field elements per digest, eight bytes each. -/
def rate : Nat := 4

/-- Bytes of direction bits for a tree of this depth, `ceil(depth / 8)`. -/
def dirBytes (depth : Nat) : Nat := (depth + 7) / 8

/-- The header length the guard requires: magic and depth byte, the sibling
digests, then the direction bitfield. -/
def header (depth : Nat) : Nat := 9 + depth * rate * 8 + dirBytes depth

/-- Byte offset of the sibling region: directly after the depth byte. -/
def sibStart (_depth : Nat) : Nat := 9

/-- Byte offset of the direction bitfield: after the siblings. -/
def dirStart (depth : Nat) : Nat := 9 + depth * rate * 8

/-- The guard the Rust applies before any sibling read. -/
def accepted (len depth : Nat) : Prop := 1 ≤ depth ∧ depth ≤ 255 ∧ header depth ≤ len

/-- The direction bitfield holds a bit for every level: reading level `i` from
byte `i / 8` stays inside the bitfield the guard reserved. -/
theorem direction_byte_in_range {depth i : Nat} (hi : i < depth) : i / 8 < dirBytes depth := by
  unfold dirBytes
  omega

/-- The direction bitfield lies within an accepted blob: it ends exactly at
the header, which the guard bounded by the length. -/
theorem direction_region_in_range {len depth : Nat} (h : accepted len depth) :
    dirStart depth + dirBytes depth ≤ len := by
  obtain ⟨_, _, hh⟩ := h
  unfold header at hh
  unfold dirStart
  omega

/-- Every sibling cell read is inside the blob. For level `level < depth` and
cell `c < rate`, the eight bytes at `(level * rate + c) * 8` past the sibling
start end at or before the header, which the guard proved is within the blob.
This is the theorem that says the parse cannot slice past the end. -/
theorem sibling_cell_in_range {len depth level c : Nat}
    (h : accepted len depth) (hl : level < depth) (hc : c < rate) :
    sibStart depth + (level * rate + c) * 8 + 8 ≤ len := by
  obtain ⟨_, _, hh⟩ := h
  unfold header rate at hh
  unfold sibStart
  unfold rate at hc ⊢
  -- level < depth gives (level + 1) * 4 <= depth * 4, and the cell ends at
  -- (level * 4 + c + 1) * 8 <= (level + 1) * 4 * 8, inside the reserved region
  have h1 : level + 1 ≤ depth := hl
  have hmul : (level + 1) * 4 ≤ depth * 4 := Nat.mul_le_mul_right 4 h1
  have hexp : (level + 1) * 4 = level * 4 + 4 := by
    rw [Nat.succ_mul]
  omega

/-- The sibling region is exactly the digests it claims to hold: the bytes
from the sibling start to the direction start are `depth * rate * 8`, no slack
and no shortfall. A parser that read a different number of bytes per digest
would either overrun or silently ignore trailing bytes; neither can happen. -/
theorem sibling_region_exact (depth : Nat) :
    dirStart depth - sibStart depth = depth * rate * 8 := by
  unfold dirStart sibStart
  omega

/-- The regions tile the header with no unparsed gap: siblings end where the
directions begin, the directions end at the header, and the proof begins
there. This is what makes the layout unambiguous, so two implementations
cannot disagree about where the proof starts. -/
theorem no_gap_before_proof {len depth : Nat} (h : accepted len depth) :
    sibStart depth + depth * rate * 8 = dirStart depth ∧
      dirStart depth + dirBytes depth = header depth ∧ header depth ≤ len := by
  obtain ⟨_, _, hh⟩ := h
  refine ⟨?_, ?_, hh⟩
  · unfold dirStart sibStart; omega
  · unfold dirStart header; omega

/-- A blob shorter than the header is rejected, so the guard is not merely
sufficient but necessary: there is no accepted length below the header. -/
theorem short_blob_rejected {len depth : Nat} (hlen : len < header depth) :
    ¬ accepted len depth := by
  intro h
  obtain ⟨_, _, hh⟩ := h
  omega

/-- Depth zero is rejected. The Rust checks this explicitly; without it,
`dirBytes 0 = 0` and the sibling region would be empty, so a trailer could
claim membership in a tree with no levels. -/
theorem zero_depth_rejected {len : Nat} : ¬ accepted len 0 := by
  intro h
  obtain ⟨h1, _, _⟩ := h
  omega

end Zkolang.Trailer
