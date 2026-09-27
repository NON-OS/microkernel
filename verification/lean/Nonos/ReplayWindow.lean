/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

How long a replayed gateway frame stays refused.

`capsule_net_nym`'s receive path authenticates a frame under the session key and
then asks `replay_gate::fresh` whether the nonce has been offered before. That
call is the whole of the replay defence on the live path, and behind it is
`state::replay::ReplayWindow`: sixty-four slots, a ring index, and a linear scan.

The order is right and worth recording: the window is consulted after
authentication, so a frame nobody could have produced cannot spend a slot. What
this file is about is the sixty-four.

A ring of sixty-four remembers the last sixty-four tags it was given and nothing
before them, so a frame is refused for as long as sixty-four further frames have
not arrived, and accepted after. `a_tag_survives_sixty_three_others` and
`sixty_four_others_evict_a_tag` are the two sides of that, computed rather than
argued, because the boundary is a specific number and a reader should be able to
see which side of it they are on.

That bound is a count of frames and not a length of time. Nothing in the window
ages, so waiting does not restore protection and neither does losing it depend on
waiting: whoever can cause sixty-four authenticated frames to pass can then
replay any frame from before them. On this path the frames come from the gateway,
so the reading is that the window protects against a passive replayer on the wire
and not against a gateway that floods its own window. Whether that is the
intended strength is a design question; the number is written down here so it is
answered on purpose.

Two things the implementation gets right that a shorter one would not. The nonce
is twelve bytes carried whole into a thirty-two byte tag, left aligned and zero
padded, never folded, so distinct nonces stay distinct: `padding_is_injective`.
And the slots carry a `used` flag rather than treating an all-zero tag as empty,
which is what lets a genuine all-zero nonce be seen for the first time;
`without_the_used_flag_a_zero_nonce_is_refused_on_sight` is what the shorter
version would have cost.
-/

set_option maxRecDepth 8000

namespace Nonos.ReplayWindow

/-! ### The shape of the tag

    `NONCE_BYTES` is 12 and `REPLAY_TAG_LEN` is 32, and `fresh` copies the nonce
    into the low end of a zeroed tag. -/

/-- `NONCE_BYTES`. -/
def nonceBytes : Nat := 12

/-- `REPLAY_TAG_LEN`. -/
def tagBytes : Nat := 32

/-- The nonce has to fit, or the copy is not a copy. -/
theorem the_nonce_fits_the_tag : nonceBytes ≤ tagBytes := by
  unfold nonceBytes tagBytes; omega

/-- `tag[..NONCE_BYTES].copy_from_slice(nonce)` over a zeroed tag. -/
def padTag (nonce : List Nat) : List Nat :=
  nonce ++ List.replicate (tagBytes - nonceBytes) 0

/-- The padded tag is a full tag. -/
theorem padding_gives_a_full_tag (nonce : List Nat) (h : nonce.length = nonceBytes) :
    (padTag nonce).length = tagBytes := by
  unfold padTag
  rw [List.length_append, List.length_replicate, h]
  unfold nonceBytes tagBytes
  omega

/-- Distinct nonces give distinct tags, so the padding invents no collisions.
    This is what the code's own comment claims, and it is the difference between
    carrying twelve bytes whole and folding them down. -/
theorem padding_is_injective (a b : List Nat)
    (ha : a.length = nonceBytes) (hb : b.length = nonceBytes)
    (h : padTag a = padTag b) : a = b := by
  unfold padTag at h
  have := List.append_inj_left h (by rw [ha, hb])
  exact this

/-! ### The window

    A ring of sixty-four that overwrites the oldest slot remembers exactly the
    last sixty-four tags it was given, so that set is the model. -/

/-- `DEPTH`. -/
def depth : Nat := 64

/-- What the window remembers, most recent first. -/
abbrev Window := List Nat

/-- `ReplayWindow::new`: nothing remembered, and no slot marked used. -/
def empty : Window := []

/-- `ReplayWindow::accept`. Refuses a tag it is holding, otherwise records it and
    drops whatever fell off the end. -/
def accept (w : Window) (t : Nat) : Bool × Window :=
  if w.contains t then (false, w) else (true, (t :: w).take depth)

/-- The window after offering a tag, whether or not it was taken. -/
def step (w : Window) (t : Nat) : Window := (accept w t).2

/-- A sequence of frames, oldest first. -/
def run (w : Window) (ts : List Nat) : Window := ts.foldl step w

/-! ### What it guarantees -/

/-- A tag nobody has offered is accepted. -/
theorem a_fresh_tag_is_accepted (w : Window) (t : Nat) (h : t ∉ w) :
    (accept w t).1 = true := by
  unfold accept
  simp [h]

/-- Offered twice with nothing in between, the second is refused. This is the
    property the gate exists for. -/
theorem an_immediate_repeat_is_refused (w : Window) (t : Nat) (h : t ∉ w) :
    (accept (step w t) t).1 = false := by
  unfold step accept
  simp [h, depth]

/-- A tag is refused exactly while it is remembered, so the question of how long
    it is refused is the question of how long it stays in the window. -/
theorem refusal_is_membership (w : Window) (t : Nat) :
    (accept w t).1 = false ↔ t ∈ w := by
  unfold accept
  by_cases h : t ∈ w <;> simp [h]

/-- A single frame leaves the window no deeper than it was allowed to be. The
    ring cannot grow, and neither can the model of it: either the tag was already
    held and nothing moved, or it was recorded and the tail was truncated. -/
theorem step_keeps_the_depth (w : Window) (t : Nat) (h : w.length ≤ depth) :
    (step w t).length ≤ depth := by
  have hcases : step w t = w ∨ step w t = (t :: w).take depth := by
    unfold step accept
    by_cases hc : t ∈ w <;> simp [hc]
  rcases hcases with hw | hw
  · rw [hw]; exact h
  · rw [hw]; exact List.length_take_le depth (t :: w)

/-- So the window never holds more than sixty-four tags, whatever sequence of
    frames it is given. -/
theorem the_window_never_exceeds_its_depth (w : Window) (ts : List Nat)
    (h : w.length ≤ depth) : (run w ts).length ≤ depth := by
  unfold run
  induction ts generalizing w with
  | nil => simpa using h
  | cons t rest ih =>
    simp only [List.foldl_cons]
    exact ih _ (step_keeps_the_depth w t h)

/-- Starting empty, that holds unconditionally. -/
theorem a_fresh_window_never_exceeds_its_depth (ts : List Nat) :
    (run empty ts).length ≤ depth := by
  apply the_window_never_exceeds_its_depth
  unfold empty depth
  simp

/-! ### Where the guarantee stops

    The boundary is a specific count, so it is computed at that count rather than
    argued about in general. Sixty-three other frames and the tag is still
    refused; sixty-four and it is accepted again. -/

/-- Sixty-three distinct frames after it, and a replay of the original is still
    refused. -/
theorem a_tag_survives_sixty_three_others :
    (accept (run (step empty 0) ((List.range 63).map (· + 1))) 0).1 = false := by
  rfl

/-- Sixty-four, and it is accepted. The window has forgotten it, so the frame can
    be replayed. -/
theorem sixty_four_others_evict_a_tag :
    (accept (run (step empty 0) ((List.range 64).map (· + 1))) 0).1 = true := by
  rfl

/-! ### The used flag

    The slots carry `used` beside the tag. Without it an all-zero tag is
    indistinguishable from an empty slot, and a zeroed window would appear to
    contain it. -/

/-- The window as it would be with zero standing for empty: sixty-four zeroed
    slots, and membership read straight off them. -/
def zeroedSlots : Window := List.replicate depth 0

/-- A genuine all-zero nonce would be refused the first time it was ever seen, so
    the flag is not defensive dressing, it is what makes that nonce usable. A
    refusal here is a frame dropped for no reason rather than a replay let
    through, which is why this is the milder of the two mistakes and still worth
    not making. -/
theorem without_the_used_flag_a_zero_nonce_is_refused_on_sight :
    (accept zeroedSlots 0).1 = false := by
  unfold accept zeroedSlots depth
  simp

/-- With the flag, which is what the code has, the same nonce is accepted the
    first time and refused after. -/
theorem with_the_used_flag_a_zero_nonce_behaves :
    (accept empty 0).1 = true ∧ (accept (step empty 0) 0).1 = false := by
  have h : (0 : Nat) ∉ empty := by unfold empty; simp
  refine ⟨a_fresh_tag_is_accepted _ _ h, an_immediate_repeat_is_refused _ _ h⟩

/-! ### Axiom profile -/

#print axioms Nonos.ReplayWindow.the_nonce_fits_the_tag
#print axioms Nonos.ReplayWindow.padding_gives_a_full_tag
#print axioms Nonos.ReplayWindow.padding_is_injective
#print axioms Nonos.ReplayWindow.a_fresh_tag_is_accepted
#print axioms Nonos.ReplayWindow.an_immediate_repeat_is_refused
#print axioms Nonos.ReplayWindow.refusal_is_membership
#print axioms Nonos.ReplayWindow.step_keeps_the_depth
#print axioms Nonos.ReplayWindow.the_window_never_exceeds_its_depth
#print axioms Nonos.ReplayWindow.a_fresh_window_never_exceeds_its_depth
#print axioms Nonos.ReplayWindow.a_tag_survives_sixty_three_others
#print axioms Nonos.ReplayWindow.sixty_four_others_evict_a_tag
#print axioms Nonos.ReplayWindow.without_the_used_flag_a_zero_nonce_is_refused_on_sight
#print axioms Nonos.ReplayWindow.with_the_used_flag_a_zero_nonce_behaves

end Nonos.ReplayWindow
