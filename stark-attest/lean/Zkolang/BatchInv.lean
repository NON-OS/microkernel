/-
 zKølang by NØNOS
 AGPL-3.0-or-later
-/
import Zkolang.Field

/-!
The prover's DEEP pass inverts a whole block of denominators with one field
inversion: prefix products forward, one inverse of the total, a backward walk
peeling one element off at a time. The i-th output is
`invT * (product after i) * (product before i)`, and multiplied by the i-th
input it is the whole product times `invT` again, whatever i is.

That is an integer identity plus one congruence, which is exactly the shape
this development's transfer principle carries: no multiplicative inverse is
constructed here, matching the field module, which leaves inversion beyond the
core library. The theorem says: hand the walk any witness that `invT` inverts
the total product, and every one of its outputs inverts its element.
-/

namespace Zkolang.BatchInv

open Zkolang.Field

/-- The product of a list, left to right, as the Rust prefix loop computes it. -/
def prodL : List Int → Int
  | [] => 1
  | x :: xs => x * prodL xs

/-- What the backward walk emits for position `i`: the inverse of the total,
times everything after `i`, times everything before `i`. -/
def outAt (invT : Int) (v : List Int) (i : Nat) : Int :=
  invT * prodL (v.drop (i + 1)) * prodL (v.take i)

/-- Splitting a product at any position: everything before, the element,
everything after. -/
theorem prod_split (v : List Int) (i : Nat) (hi : i < v.length) :
    prodL v = prodL (v.take i) * v[i] * prodL (v.drop (i + 1)) := by
  induction v generalizing i with
  | nil => cases hi
  | cons x xs ih =>
    cases i with
    | zero =>
      simp [prodL, List.take, List.drop]
    | succ n =>
      have hn : n < xs.length := Nat.lt_of_succ_lt_succ hi
      simp only [List.take, List.drop, List.getElem_cons_succ, prodL]
      rw [ih n hn]
      rw [Int.mul_assoc, Int.mul_assoc, Int.mul_assoc]

/-- The walk is correct at every position: if `invT` inverts the whole product,
each output inverts its element. One integer rearrangement, one congruence. -/
theorem walk_inverts (invT : Int) (v : List Int)
    (h : cong (invT * prodL v) 1) (i : Nat) (hi : i < v.length) :
    cong (outAt invT v i * v[i]) 1 := by
  have hint : outAt invT v i * v[i] = invT * prodL v := by
    unfold outAt
    rw [prod_split v i hi]
    ac_rfl
  exact cong_trans (transfer hint) h

end Zkolang.BatchInv
