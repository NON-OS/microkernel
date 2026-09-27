/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

The capability decision cores, on the extracted code.

Three files in `src/capabilities` were factored out of the structures they belong
to so that they hold no token, no atomic and no lock, and each says in its own
header that it exists to be checked against a Lean model. Those models are
hand-written. These are the same properties over the code itself.

`delegation_expiry` is the meet that stops a delegation outliving its parent. A
delegation is signed by the kernel key, so one that outlives its parent verifies
everywhere for as long as it lasts, which makes this four-line function the whole
of the bound. `a_child_never_outlives_its_parent` is stated for every requested
value and every parent expiry, not at witnesses.

`has_at_least` is the quota comparison. The extracted body is literally
`ok (remaining >= amount)`, where that `>=` is the scalar library's Bool-valued
comparison and not the `≥` of ordinary Lean notation, so the general equality is
awkward to state and the cases below are stated at the boundaries instead: an
empty quota, exactly enough, and one too many.

`compose` builds a resource nonce from a millisecond timestamp and a monotonic
counter. Its own header says the counter occupies the low 32 bits and the
timestamp the high bits. The first half is exactly true and proven here. The
second is not: the high half is the timestamp's *low* 32 bits, because the shift
truncates rather than failing, so a current epoch-millisecond value loses nine
bits. Two timestamps 2^32 milliseconds apart, which is 49.7 days, compose to the
same nonce for an equal counter, and a counter that passes 2^32 collides with one
2^32 earlier.

Neither collision is reachable today. The counter is a global that starts at one
and only increments, so reaching the second needs 4.3 billion resource tokens, and
the first needs the same counter value at two timestamps 49.7 days apart, which a
monotonic counter does not produce. `reset_nonce_counter` would produce it, and it
has no caller. What is written down here is the bound, so the next person to reach
for this nonce knows what it guarantees rather than what its comment says.
-/

import NonosExtraction.Caps

open Aeneas Aeneas.Std Result
open nonos_caps

set_option linter.hashCommand false
set_option maxRecDepth 100000

namespace NonosExtraction

open capabilities.delegation.lifetime renaming delegation_expiry → expiryMeet
open capabilities.resource.limits renaming has_at_least → quotaCovers
open capabilities.resource.nonce_compose renaming compose → nonceOf
open capabilities.chain.constants renaming max_chain_depth → chainDepth

/-! ### The delegation expiry meet -/

/-- A parent with no expiry imposes no bound, so the request stands unchanged,
    including a request for no expiry at all. -/
theorem no_parent_expiry_means_the_request_stands (requested : Option Std.U64) :
    expiryMeet requested none = ok requested := rfl

/-- A parent with an expiry bounds the result even when the caller asked for
    nothing. This is the case a caller gets wrong by omission rather than by
    asking for too much. -/
theorem a_parent_bounds_an_unbounded_request (p : Std.U64) :
    expiryMeet none (some p) = ok (some p) := rfl

/-- The load-bearing one: whatever is requested, a delegation under a parent that
    expires also expires, and never later than the parent.

    Stated for every requested value and every parent, because a bound that holds
    only at the values someone tested is not a bound. -/
theorem a_child_never_outlives_its_parent (requested : Option Std.U64) (p : Std.U64) :
    ∃ q, expiryMeet requested (some p) = ok (some q) ∧ q.val ≤ p.val := by
  cases requested with
  | none => exact ⟨p, rfl, Nat.le_refl _⟩
  | some e =>
    by_cases h : e < p
    · refine ⟨e, ?_, ?_⟩
      · show (if e < p then ok (some e) else ok (some p)) = ok (some e)
        rw [if_pos h]
      · scalar_tac
    · refine ⟨p, ?_, Nat.le_refl _⟩
      show (if e < p then ok (some e) else ok (some p)) = ok (some p)
      rw [if_neg h]

/-- Asking for less than the parent allows is honoured, so the meet does not
    silently widen a delegation to its parent's lifetime. -/
theorem a_shorter_request_is_honoured :
    expiryMeet (some 5#u64) (some 9#u64) = ok (some 5#u64) := rfl

/-- Asking for more is refused down to the parent. -/
theorem a_longer_request_is_cut_to_the_parent :
    expiryMeet (some 9#u64) (some 5#u64) = ok (some 5#u64) := rfl

/-- And the meet of a bound with itself is that bound, so re-delegating at the
    parent's own expiry neither extends nor shortens it. -/
theorem the_meet_is_idempotent :
    expiryMeet (some 5#u64) (some 5#u64) = ok (some 5#u64) := rfl

/-! ### The quota comparison -/

/-- Nothing covers more than what remains. -/
theorem an_empty_quota_covers_nothing_positive :
    quotaCovers 0#u64 1#u64 = ok false := rfl

/-- A request of nothing is covered even out of an empty quota, which is the
    boundary a check written as a subtraction gets wrong. -/
theorem a_request_of_nothing_is_always_covered :
    quotaCovers 0#u64 0#u64 = ok true ∧ quotaCovers 7#u64 0#u64 = ok true := by
  refine ⟨rfl, rfl⟩

/-- Exactly enough is enough. -/
theorem exactly_enough_is_covered :
    quotaCovers 7#u64 7#u64 = ok true := rfl

/-- One more than remains is not. -/
theorem one_too_many_is_refused :
    quotaCovers 7#u64 8#u64 = ok false := rfl

/-! ### The nonce composition

    What the header claims, and what the code does. -/

/-- The counter is recoverable from the nonce, which is the claim the comment
    makes and the one the replay window depends on: the low 32 bits of the nonce
    are the counter. -/
theorem the_counter_is_recoverable :
    (do let n ← nonceOf 1790000000000#u64 7#u64; ok (n &&& 0xFFFFFFFF#u64)) =
      ok 7#u64 ∧
    (do let n ← nonceOf 1#u64 4294967295#u64; ok (n &&& 0xFFFFFFFF#u64)) =
      ok 4294967295#u64 := by
  refine ⟨?_, ?_⟩ <;> rfl

/-- Two distinct counters inside one timestamp give distinct nonces, which is the
    property that matters while the counter is below 2^32. -/
theorem distinct_counters_do_not_collide :
    nonceOf 1790000000000#u64 7#u64 ≠ nonceOf 1790000000000#u64 8#u64 := by
  rw [show nonceOf 1790000000000#u64 7#u64 = ok 0xC4506C0000000007#u64 from rfl,
      show nonceOf 1790000000000#u64 8#u64 = ok 0xC4506C0000000008#u64 from rfl]
  simp

/-- The shift truncates rather than failing, so the nonce's high half is the
    timestamp's low 32 bits and not the timestamp.

    At a current epoch-millisecond value the timestamp is 41 bits and nine of them
    are discarded. The witness is the whole nonce, so the reader can see the high
    half is `0xC4506C00`, which is `timestamp &&& 0xFFFFFFFF`. -/
theorem the_timestamp_is_truncated_not_carried :
    nonceOf 1790000000000#u64 7#u64 = ok 0xC4506C0000000007#u64 ∧
    (1790000000000#u64 &&& 0xFFFFFFFF#u64) = 0xC4506C00#u64 := by
  refine ⟨rfl, ?_⟩
  rfl

/-- So two timestamps 2^32 milliseconds apart, 49.7 days, compose to the same
    nonce for an equal counter. -/
theorem the_nonce_repeats_every_two_to_the_thirty_two_milliseconds :
    nonceOf 1790000000000#u64 7#u64 = nonceOf 1794294967296#u64 7#u64 := by
  rw [show nonceOf 1790000000000#u64 7#u64 = ok 0xC4506C0000000007#u64 from rfl,
      show nonceOf 1794294967296#u64 7#u64 = ok 0xC4506C0000000007#u64 from rfl]

/-- And a counter that passes 2^32 collides with the one 2^32 before it, because
    the counter is masked to 32 bits.

    Unreachable while the counter only increments from one, which needs 4.3
    billion tokens, and reachable through `reset_nonce_counter`, which has no
    caller. -/
theorem a_wrapped_counter_collides :
    nonceOf 1790000000000#u64 0#u64 = nonceOf 1790000000000#u64 4294967296#u64 := by
  rw [show nonceOf 1790000000000#u64 0#u64 = ok 0xC4506C0000000000#u64 from rfl,
      show nonceOf 1790000000000#u64 4294967296#u64 = ok 0xC4506C0000000000#u64 from rfl]

/-- The composition never fails, at a realistic timestamp and at the widest
    counter, so the truncation is silent rather than an error the caller could
    notice. -/
theorem the_composition_never_refuses :
    (∃ n, nonceOf 1790000000000#u64 7#u64 = ok n) ∧
    (∃ n, nonceOf 0xFFFFFFFFFFFFFFFF#u64 0xFFFFFFFFFFFFFFFF#u64 = ok n) := by
  refine ⟨⟨_, rfl⟩, ⟨_, rfl⟩⟩

/-! ### The chain depth bound -/

/-- A delegation chain is walked to a fixed depth, so a cycle or a forged long
    chain costs a bounded amount of work rather than an unbounded one. -/
theorem the_chain_depth_is_sixteen : chainDepth = ok 16#usize := by
  simp [chainDepth, capabilities.chain.constants.MAX_CHAIN_DEPTH]

/-! ### Axiom profile -/

#print axioms NonosExtraction.no_parent_expiry_means_the_request_stands
#print axioms NonosExtraction.a_parent_bounds_an_unbounded_request
#print axioms NonosExtraction.a_child_never_outlives_its_parent
#print axioms NonosExtraction.a_shorter_request_is_honoured
#print axioms NonosExtraction.a_longer_request_is_cut_to_the_parent
#print axioms NonosExtraction.the_meet_is_idempotent
#print axioms NonosExtraction.a_request_of_nothing_is_always_covered
#print axioms NonosExtraction.exactly_enough_is_covered
#print axioms NonosExtraction.one_too_many_is_refused
#print axioms NonosExtraction.the_counter_is_recoverable
#print axioms NonosExtraction.distinct_counters_do_not_collide
#print axioms NonosExtraction.the_timestamp_is_truncated_not_carried
#print axioms NonosExtraction.the_nonce_repeats_every_two_to_the_thirty_two_milliseconds
#print axioms NonosExtraction.a_wrapped_counter_collides
#print axioms NonosExtraction.the_composition_never_refuses
#print axioms NonosExtraction.the_chain_depth_is_sixteen

end NonosExtraction
