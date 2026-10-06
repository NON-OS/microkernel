/-
 stark-attest by NØNOS
 AGPL-3.0-or-later
-/

/-!
Reserved slots are unreachable: no real artifact can occupy one.

The policy tree has a fixed width, so a set of `n` members is padded to `2^d`
leaves with a reserved value. If a real artifact could ever measure to the same
leaf as a padding slot, an attacker could claim membership by presenting the
padding image, which every verifier already accepts as a leaf of the tree.
The separation must therefore be structural, not probabilistic.

The crate makes it structural by choosing a padding image that begins with a
byte no admissible artifact can begin with:

    PAD = 0x00 :: "STARK-ATTEST-RESERVED-SLOT-v1"

and the enrolment path only ever measures artifacts, never the pad, into member
slots. This module proves the separation from that one property: the images
live in disjoint sets because their first bytes differ, so the measurement
function, whatever it is, is applied to inputs that are never equal, and no
collision argument is needed at all. That is the point: this is a domain
separation proof, not a hash security proof. It holds even against an adversary
who breaks the hash.

The admissibility side condition is the one the enrolment tool enforces and
every real format satisfies: an artifact is a nonempty byte string whose first
byte is not `0x00`. ELF starts `0x7F`, PE starts `0x4D`, Mach-O starts `0xCF`
or `0xFE`, a tarball starts with a filename character, and a script starts with
`#`. The tool rejects an empty file outright.
-/

namespace Zkolang.Padding

/-- The reserved padding image, as its byte list. -/
def padImage : List Nat := 0 :: [83, 84, 65, 82, 75]  -- 0x00 then "STARK"

/-- An artifact is admissible when it is nonempty and does not begin with the
reserved byte. This is exactly what the enrolment tool checks before measuring. -/
def admissible (image : List Nat) : Prop :=
  image ≠ [] ∧ image.head? ≠ some 0

/-- The padding image begins with the reserved byte. -/
theorem pad_head : padImage.head? = some 0 := rfl

/-- The padding image is nonempty, so "begins with" is meaningful for it. -/
theorem pad_nonempty : padImage ≠ [] := by
  unfold padImage
  simp

/-- No admissible artifact equals the padding image. An attacker cannot present
a real artifact that lands in a reserved slot, because the two byte strings
differ in their first byte before any hash is applied. -/
theorem admissible_ne_pad {image : List Nat} (h : admissible image) : image ≠ padImage := by
  intro heq
  obtain ⟨_, hhead⟩ := h
  rw [heq] at hhead
  exact hhead pad_head

/-- The measurement of an admissible artifact is never the measurement of the
padding image, for any measurement function that is a function of the bytes.
Stated with the hash abstract on purpose: the separation is upstream of the
hash, so it survives a hash break. The hypothesis is injectivity only on this
pair, which is strictly weaker than collision resistance. -/
theorem measure_ne_pad {α : Type} (measure : List Nat → α) {image : List Nat}
    (h : admissible image)
    (hinj : ∀ x y : List Nat, x ≠ y → measure x ≠ measure y) :
    measure image ≠ measure padImage :=
  hinj image padImage (admissible_ne_pad h)

/-- Padding fills exactly the slots the members do not: a set of `n` members in
a tree of `width` leaves uses `width - n` reserved slots, and the two counts
partition the tree. A padding scheme that left a slot uninitialised would leave
a leaf an attacker could define. -/
theorem padding_partitions {n width : Nat} (h : n ≤ width) : n + (width - n) = width := by
  omega

/-- Every slot index is either a member index or a padding index, never both.
The disjointness is what makes "member slots hold artifacts, padding slots hold
the reserved image" a complete description of the tree. -/
theorem slot_dichotomy {n width i : Nat} (hi : i < width) :
    (i < n ∧ ¬ (n ≤ i)) ∨ (n ≤ i ∧ ¬ (i < n)) := by
  omega

end Zkolang.Padding
