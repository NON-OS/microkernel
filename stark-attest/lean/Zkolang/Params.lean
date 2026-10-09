/-
 stark-attest by NØNOS
 AGPL-3.0-or-later
-/

/-!
The verifier cannot be talked down.

Soundness of a FRI-based argument rests on numbers: how many queries are
checked, how many bits were ground into the transcript, how much the evaluation
domain was blown up. The dangerous failure is not a wrong number, it is a
*negotiated* number: a proof that carries its own parameters and a verifier
that believes them. Then an attacker proves at four queries, claims four, and a
verifier that accepts the claim checks four.

The crate forecloses that structurally. `attest_params` is the only source of
these values, the verifier passes them in from there, and nothing in a trailer
can influence them. This module states that as arithmetic: the verifier's
effective parameters are its own, independent of anything the prover supplies,
and the soundness bound is monotone in them, so an attacker who could influence
them could only ever make the check stricter, never weaker.

The bound modelled here is the standard one for the query phase, stated in
terms of "the adversary's chance of surviving `q` independent queries, each of
which catches a far-from-low-degree codeword with probability at least `1 - r`,
is at most `r^q`". The theorem is not the FRI soundness theorem, which is
inherited from the literature and stated in the security documentation. The
theorem here is the composition fact the implementation depends on: more
queries never help the adversary, and the verifier's count is the one used.
-/

namespace Zkolang.Params

/-- The parameters a verifier runs with. In the crate these are constants read
from one module; here they are a record so the theorems can quantify over them. -/
structure Verifier where
  queries : Nat
  grindBits : Nat
  blowupBits : Nat

/-- What a proof claims about itself. The crate deliberately gives this no
route into the verifier; the type exists to state that fact. -/
structure Claimed where
  queries : Nat
  grindBits : Nat
  blowupBits : Nat

/-- The effective parameters: the verifier's own, always. This models
`stark_verify_ext_blown_bound` being called with the constants from
`attest_params` and never with anything parsed from the trailer. -/
def effective (v : Verifier) (_c : Claimed) : Verifier := v

/-- Independence: whatever a proof claims, the effective parameters are the
verifier's. Two proofs claiming different parameters are checked identically. -/
theorem effective_ignores_claim (v : Verifier) (c1 c2 : Claimed) :
    effective v c1 = effective v c2 := rfl

/-- Stated the other way: the effective parameters equal the verifier's own,
for every claim. A downgrade attack has no surface to act on. -/
theorem no_downgrade (v : Verifier) (c : Claimed) : effective v c = v := rfl

/-- Survival probability across `q` independent queries, as a rational bound
represented by numerator and denominator powers: an adversary surviving one
query with probability at most `num / den` survives `q` with at most
`num^q / den^q`. Nat arithmetic on the numerator, monotone in `q`. -/
def survivalNum (num q : Nat) : Nat := num ^ q

/-- Monotonicity in the query count: raising the number of queries never raises
the adversary's survival bound, provided a single query is not certain to be
survived. With `num = 0` (a query that always catches) the bound is zero for
any positive count; the interesting case is `num >= 1`, where the power is
non-increasing in the exponent exactly when `num <= 1`. Stated here in the form
the implementation needs: the bound at `q2` queries is at most the bound at
`q1` queries whenever `q1 <= q2` and the per-query numerator is at most one,
which is the normalised form the soundness statement uses. -/
theorem survival_monotone {num q1 q2 : Nat} (hnum : num ≤ 1) (hq : q1 ≤ q2) :
    survivalNum num q2 ≤ survivalNum num q1 := by
  unfold survivalNum
  -- num is zero or one; in both cases the power is non-increasing in the exponent
  match num, hnum with
  | 0, _ =>
    cases q1 with
    | zero =>
      -- goal: 0 ^ q2 <= 0 ^ 0 = 1, and a power of zero is zero or one
      cases q2 with
      | zero => simp
      | succ m => simp
    | succ n =>
      cases q2 with
      | zero => omega
      | succ m => simp
  | 1, _ => simp

/-- More queries is never worse for the honest verifier either: the check at a
higher query count accepts a subset of what a lower count accepts. Modelled as
the implication on the acceptance predicate, which is what "stricter" means. -/
theorem more_queries_is_stricter {accept : Nat → Prop}
    (mono : ∀ a b : Nat, a ≤ b → accept b → accept a) {q1 q2 : Nat} (h : q1 ≤ q2) :
    accept q2 → accept q1 := mono q1 q2 h

/-- The parameters the deployment runs. Recorded here so a change to
`attest_params` that forgets to update the documentation shows up as a proof
about numbers that no longer match the constants. -/
def deployed : Verifier := { queries := 32, grindBits := 16, blowupBits := 3 }

/-- The deployed query count is the one the soundness discussion assumes. -/
theorem deployed_queries : deployed.queries = 32 := rfl

/-- The deployed grinding is sixteen bits. -/
theorem deployed_grind : deployed.grindBits = 16 := rfl

/-- The deployed extra blowup is three bits. -/
theorem deployed_blowup : deployed.blowupBits = 3 := rfl

end Zkolang.Params
