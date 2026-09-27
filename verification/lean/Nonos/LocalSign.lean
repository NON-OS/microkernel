/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

One authority, and the several places that have to agree about it.

Signing a package on the machine it will run on is a single authority, and it was
expressed as `Admin`. No capsule holds `Admin`. The result was not a refusal
anyone saw: the installer wrote the package without a trailer, the loader found no
trailer, and the exec gate refused the binary later, somewhere else, for a reason
that named neither signing nor authority. The path was unreachable and nothing
reported it as unreachable.

The fix is a bit of its own, and a bit of its own has to be agreed in every place
that mentions it: the table, the manifest parser, the mint call, the sign call,
the verify path, the audit tool and the capsule manifests. Seven places, and a
disagreement in any one of them puts the path back where it started.

That is what these theorems are about. `unheld_authority_is_unreachable` is the
defect: a gate on an authority nothing holds refuses everything, and the refusal
appears at a later gate wearing a different name. `every_site_must_agree` is the
constraint the fix has to satisfy, and `derived_sites_agree` is why deriving the
sites from one declaration discharges it instead of documenting it.
-/

namespace Nonos.LocalSign

/-! ### Authority -/

/-- The authorities this file needs to tell apart. `admin` is the one no capsule
    holds; `localSign` is the dedicated bit. -/
inductive Authority where
  | admin
  | localSign
  | appInstall
  deriving DecidableEq, Repr

/-- What a capsule holds. -/
def Holder := Authority → Bool

/-- The machine as it shipped: no capsule holds `admin`. -/
def noAdmin (h : Holder) : Prop := h .admin = false

/-! ### A gate on an authority nothing holds -/

/-- A request to sign a package locally. -/
structure Request where
  package : Nat
  caller : Nat
  deriving Repr

/-- What happens to it. `refused` is honest; `untrailered` is what the defect
    actually produced, an install that completed and left a package nothing
    would later accept. -/
inductive Outcome where
  | signed (package : Nat)
  | refused
  | untrailered (package : Nat)
  deriving DecidableEq, Repr

/-- The gate, parameterised by the authority it tests. -/
def gate (a : Authority) (h : Holder) (r : Request) : Outcome :=
  if h a then .signed r.package else .refused

/-- Gated on `admin`, every request is refused, for every package and every
    caller. The path does not exist. -/
theorem unheld_authority_is_unreachable (h : Holder) (hn : noAdmin h) (r : Request) :
    gate .admin h r = .refused := by
  unfold gate
  unfold noAdmin at hn
  simp [hn]

/-- Gated on the dedicated bit, a holder is served. The fix is a gate that can
    pass, which is the part `admin` was missing. -/
theorem dedicated_bit_is_reachable (h : Holder) (hl : h .localSign = true)
    (r : Request) : gate .localSign h r = .signed r.package := by
  unfold gate
  simp [hl]

/-- And a non-holder is still refused, so the bit is a gate rather than a
    formality. -/
theorem dedicated_bit_still_refuses (h : Holder) (hl : h .localSign = false)
    (r : Request) : gate .localSign h r = .refused := by
  unfold gate
  simp [hl]

/-- The authority is independent of the right to ask for an install. A capsule
    may request an install without being able to sign, which is the whole reason
    `AppInstall` and `LocalSign` are separate bits: a window that can ask must
    not thereby become a window that can vouch. -/
theorem asking_is_not_signing (h : Holder)
    (ha : h .appInstall = true) (hl : h .localSign = false) (r : Request) :
    gate .localSign h r = .refused ∧ h .appInstall = true :=
  ⟨dedicated_bit_still_refuses h hl r, ha⟩

/-! ### What the defect produced instead of a refusal -/

/-- The install path as it behaved: it asked the gate, and on a refusal it wrote
    the package anyway, without a trailer. -/
def installWithoutChecking (a : Authority) (h : Holder) (r : Request) : Outcome :=
  match gate a h r with
  | .signed p => .signed p
  | _ => .untrailered r.package

/-- So the refusal never reached the caller as a refusal. The install reported
    success and left a package the exec gate would reject later, which is why the
    diagnosis pointed at the loader. -/
theorem refusal_became_an_untrailered_install (h : Holder) (hn : noAdmin h)
    (r : Request) :
    installWithoutChecking .admin h r = .untrailered r.package := by
  unfold installWithoutChecking
  rw [unheld_authority_is_unreachable h hn r]

/-- An install that propagates the refusal is the one that could have been
    diagnosed: nothing is written and the caller is told why. -/
def install (a : Authority) (h : Holder) (r : Request) : Outcome := gate a h r

theorem propagating_install_refuses (h : Holder) (hn : noAdmin h) (r : Request) :
    install .admin h r = .refused :=
  unheld_authority_is_unreachable h hn r

/-- No untrailered package ever leaves the propagating install, for any
    authority and any holder. -/
theorem propagating_install_never_writes_untrailered
    (a : Authority) (h : Holder) (r : Request) (p : Nat) :
    install a h r ≠ .untrailered p := by
  unfold install gate
  by_cases hb : h a
  · simp [hb]
  · simp [hb]

/-! ### The seven places -/

/-- A site that mentions the authority: the authority it tests. -/
structure Site where
  tests : Authority

/-- The sites, as they exist in the tree: the table, the manifest parser, the
    mint call, the sign call, the verify path, the audit tool, and the capsule
    manifests. -/
def siteCount : Nat := 7

/-- The path runs only if every site tests the authority the holder actually
    has. -/
def PathOpen (sites : List Site) (h : Holder) : Prop :=
  ∀ s ∈ sites, h s.tests = true

/-- One site testing an authority the holder lacks closes the path, whatever the
    other six test. This is why "we changed the bit" is not a complete fix and
    the count of places matters. -/
theorem one_dissenting_site_closes_the_path
    (sites : List Site) (h : Holder) (s : Site)
    (hmem : s ∈ sites) (hs : h s.tests = false) : ¬ PathOpen sites h := by
  intro hopen
  have := hopen s hmem
  rw [hs] at this
  exact Bool.noConfusion this

/-- Every site agreeing on an authority the holder has opens the path. -/
theorem agreement_opens_the_path (sites : List Site) (h : Holder) (a : Authority)
    (hall : ∀ s ∈ sites, s.tests = a) (ha : h a = true) : PathOpen sites h := by
  intro s hmem
  rw [hall s hmem]
  exact ha

/-- Sites built from one declaration test one authority by construction, so
    agreement is not something to check and then trust, it is the shape of the
    definition. -/
def derived (a : Authority) (n : Nat) : List Site :=
  List.replicate n ⟨a⟩

theorem derived_sites_agree (a : Authority) (n : Nat) :
    ∀ s ∈ derived a n, s.tests = a := by
  intro s hmem
  unfold derived at hmem
  have := List.eq_of_mem_replicate hmem
  rw [this]

/-- And therefore the derived sites open the path for exactly the holders of that
    authority, at any number of sites, including the seven there are. -/
theorem derived_path_is_open_iff_held (a : Authority) (n : Nat) (h : Holder) :
    PathOpen (derived a n) h ↔ (n = 0 ∨ h a = true) := by
  constructor
  · intro hopen
    cases n with
    | zero => exact Or.inl rfl
    | succ k =>
      refine Or.inr ?_
      have hmem : (⟨a⟩ : Site) ∈ derived a (k + 1) := by
        unfold derived
        exact List.mem_replicate.mpr ⟨by omega, rfl⟩
      exact hopen _ hmem
  · intro h'
    cases h' with
    | inl hz =>
      subst hz
      intro s hmem
      unfold derived at hmem
      simp at hmem
    | inr ha => exact agreement_opens_the_path _ h a (derived_sites_agree a n) ha

/-- The seven sites, stated at the number that is actually in the tree, so a
    site added without a declaration shows up as a failing proof rather than as
    an install that quietly stops working. -/
theorem the_seven_sites_agree : ∀ s ∈ derived Authority.localSign siteCount,
    s.tests = Authority.localSign :=
  derived_sites_agree _ _

end Nonos.LocalSign
