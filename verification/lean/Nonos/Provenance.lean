/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

What the machine will vouch for.

Packages arrived over plain HTTP, the index signature was not checked, and the
per-file checksum in the index was parsed and not compared. Three separate
omissions with one effect: bytes of unknown origin were installed and then
attested, and the attestation was true about the bytes and silent about where
they came from. An attestation of unauthenticated bytes is worse than no
attestation, because something downstream believes it.

Provenance is therefore a value the pipeline carries, not a check it performs
once. The order below is `unauthenticated < signed < verified`, and the rules are
that fetching starts at the bottom, each check can only raise a level it has
evidence for, and combining two inputs takes the lower of the two. That last one
is the property that matters: unpacking a verified index to produce a file whose
checksum was never compared yields an unverified file, and nothing further in the
pipeline can undo that.

`unauthenticated_is_absorbing` and `no_laundering` are the two theorems the audit
should read. The first says a single unauthenticated input drags the whole result
down. The second says no sequence of steps raises a level without the
corresponding check, so provenance cannot be washed by passing bytes through
enough stages.
-/

namespace Nonos.Provenance

/-! ### The order -/

/-- How well the machine knows where some bytes came from. -/
inductive Level where
  /-- Fetched, and nothing more is known. -/
  | unauthenticated
  /-- Covered by a signature the machine checked against an enrolled key. -/
  | signed
  /-- Signed, and the content hash in the signed index matches these bytes. -/
  | verified
  deriving DecidableEq, Repr

/-- The order as a number, so comparisons are arithmetic rather than a case
    table. -/
def rank : Level → Nat
  | .unauthenticated => 0
  | .signed => 1
  | .verified => 2

def Le (a b : Level) : Prop := rank a ≤ rank b

instance (a b : Level) : Decidable (Le a b) := by
  unfold Le; infer_instance

theorem le_refl (a : Level) : Le a a := Nat.le_refl _

theorem le_trans {a b c : Level} (h1 : Le a b) (h2 : Le b c) : Le a c :=
  Nat.le_trans h1 h2

/-- Rank determines the level, so nothing is lost by reasoning about numbers. -/
theorem rank_injective {a b : Level} (h : rank a = rank b) : a = b := by
  cases a <;> cases b <;> first | rfl | (exfalso; exact absurd h (by decide))

/-- Unauthenticated is the bottom. -/
theorem unauthenticated_is_bottom (a : Level) : Le .unauthenticated a := by
  cases a <;> decide

/-- Verified is the top. -/
theorem verified_is_top (a : Level) : Le a .verified := by
  cases a <;> decide

/-! ### Combining -/

/-- Two inputs combine at the lower of their levels. -/
def meet (a b : Level) : Level := if rank a ≤ rank b then a else b

theorem meet_le_left (a b : Level) : Le (meet a b) a := by
  cases a <;> cases b <;> decide

theorem meet_le_right (a b : Level) : Le (meet a b) b := by
  cases a <;> cases b <;> decide

/-- A single unauthenticated input makes the result unauthenticated, whatever
    else went into it.

    This is what the unpack path needed: a verified index does not make a file
    verified, because the file's own checksum is a second input and an unread
    checksum is no input at all. -/
theorem unauthenticated_is_absorbing (a : Level) :
    meet .unauthenticated a = .unauthenticated ∧ meet a .unauthenticated = .unauthenticated := by
  constructor
  · cases a <;> decide
  · cases a <;> decide

/-- Combining never raises either input, so a pipeline that folds provenance
    across its stages is monotone downward by construction. -/
theorem meet_never_raises (a b : Level) : Le (meet a b) a ∧ Le (meet a b) b :=
  ⟨meet_le_left a b, meet_le_right a b⟩

/-! ### The checks, and what each one is allowed to conclude -/

/-- The evidence a stage has. -/
structure Evidence where
  /-- The index's signature verified against an enrolled key. -/
  indexSigned : Bool
  /-- The bytes hash to the digest the signed index gives for them. -/
  digestMatches : Bool
  deriving DecidableEq, Repr

/-- The level a stage may claim, given its evidence. Both checks are required for
    `verified`, the signature alone gives `signed`, and no check gives nothing. -/
def conclude (e : Evidence) : Level :=
  if e.indexSigned then
    if e.digestMatches then .verified else .signed
  else .unauthenticated

/-- Without a signature nothing is concluded, whatever the digest did. A digest
    match against an unsigned index is a match against an attacker's number. -/
theorem no_signature_concludes_nothing (e : Evidence) (h : e.indexSigned = false) :
    conclude e = .unauthenticated := by
  unfold conclude; simp [h]

/-- Verified requires both checks, so neither can be dropped as redundant. -/
theorem verified_requires_both (e : Evidence) (h : conclude e = .verified) :
    e.indexSigned = true ∧ e.digestMatches = true := by
  unfold conclude at h
  by_cases hs : e.indexSigned
  · by_cases hd : e.digestMatches
    · exact ⟨hs, hd⟩
    · simp [hs, hd] at h
  · simp [hs] at h

/-- A signature without a digest match reaches `signed` and no further: the index
    is authentic and these particular bytes are not covered by it. That is the
    exact state a fetched file was in while being treated as verified. -/
theorem signature_alone_stops_at_signed (e : Evidence)
    (hs : e.indexSigned = true) (hd : e.digestMatches = false) :
    conclude e = .signed := by
  unfold conclude; simp [hs, hd]

/-! ### The pipeline -/

/-- A stage: what it concludes from its own evidence, applied to what it was
    handed. The result is the lower of the two, which is the rule that makes the
    pipeline honest. -/
def stage (input : Level) (e : Evidence) : Level := meet input (conclude e)

/-- No stage raises the level it was handed. -/
theorem stage_never_raises (input : Level) (e : Evidence) : Le (stage input e) input :=
  meet_le_left _ _

/-- Run a whole pipeline. -/
def pipeline (input : Level) : List Evidence → Level
  | [] => input
  | e :: rest => pipeline (stage input e) rest

/-- However many stages a package passes through, its provenance never rises.
    Bytes cannot be laundered by being handled repeatedly. -/
theorem no_laundering (input : Level) (es : List Evidence) :
    Le (pipeline input es) input := by
  induction es generalizing input with
  | nil => exact le_refl input
  | cons e rest ih =>
    exact le_trans (ih (stage input e)) (stage_never_raises input e)

/-- And once unauthenticated, always unauthenticated. -/
theorem unauthenticated_stays (es : List Evidence) :
    pipeline .unauthenticated es = .unauthenticated := by
  induction es with
  | nil => rfl
  | cons e rest ih =>
    unfold pipeline stage
    rw [(unauthenticated_is_absorbing (conclude e)).1]
    exact ih

/-! ### What the machine refuses -/

/-- Execution requires verified provenance: the signature checked and the bytes
    matched. -/
def MayExecute (l : Level) : Prop := l = .verified

instance (l : Level) : Decidable (MayExecute l) := by
  unfold MayExecute; infer_instance

/-- Unauthenticated bytes never run. -/
theorem unauthenticated_never_executes : ¬ MayExecute .unauthenticated := by
  unfold MayExecute; simp

/-- Nor do bytes covered only by a signature that did not name them. -/
theorem signed_but_unmatched_never_executes : ¬ MayExecute .signed := by
  unfold MayExecute; simp

/-- And a package that entered the pipeline unauthenticated never runs, whatever
    happened to it afterwards. This is the statement the install path was
    missing. -/
theorem fetched_without_authentication_never_runs (es : List Evidence) :
    ¬ MayExecute (pipeline .unauthenticated es) := by
  rw [unauthenticated_stays es]
  exact unauthenticated_never_executes

/-- Attestation is held to the same level as execution, so the machine does not
    sign a measurement of bytes it could not place. -/
def MayAttest (l : Level) : Prop := MayExecute l

theorem never_attests_unauthenticated (es : List Evidence) :
    ¬ MayAttest (pipeline .unauthenticated es) :=
  fetched_without_authentication_never_runs es

end Nonos.Provenance
