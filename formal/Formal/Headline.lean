/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

The five properties, each stated over every reachable state of the model in
`Formal.Step`, for every platform (every signature primitive, every image
ceiling, every boot profile, every allowlist). formal/README.md maps each one
to the Rust it abstracts and lists what the model leaves out.
-/

import Formal.Proofs.WX
import Formal.Proofs.Isolation
import Formal.Proofs.Gate
import Formal.Proofs.AdminGrant
import Formal.Proofs.Envelope

namespace Formal

/-! ### 1. Capability monotonicity -/

/-- **Capability monotonicity.** Under the shipped grant rule, in every
    reachable state a capsule holds nothing outside what its manifest
    declared (required and optional) under the image ceiling: never a bit
    its manifest did not name, never a bit the ceiling did not allow, never a
    bit its publisher's certificate ceiling excludes. The word stays inside
    the envelope recorded at its spawn. No hypothesis on Admin is needed:
    a grant outside the target's envelope is refused. -/
theorem capability_monotonicity {P : Platform} {s : State} (hr : Reachable P .targetEnvelope s)
    {p : Pid} {c : Capsule} (hc : s.caps p = some c) :
    Within c.held (declared P c.request) ∧
    Within c.held c.request.cert.ceiling ∧
    Within c.held P.imageCeiling ∧
    Within c.held c.envelope := by
  obtain ⟨_, _, _, _, h5⟩ := capInv_reachable s hr p c hc
  obtain ⟨e1, e2⟩ := envInv_reachable s hr p c hc
  have hd : Within c.held (declared P c.request) :=
    within_trans e1 (e2 ▸ envelope_within_declared h5)
  obtain ⟨_, a2, _⟩ := admission_spec h5
  exact ⟨hd, within_trans hd (within_trans (within_and_left _ _) a2),
    within_trans hd (within_and_right _ _), e1⟩

/-- Under either grant rule: a capsule holds nothing beyond its admitted word
    plus what grants added, the added bits are under the image ceiling, and
    the admitted word is under both ceilings. This is the strongest bound the
    old rule (`GrantRule.callerOnly`) supported; see
    `AdminGrant.old_rule_grant_exceeds_admitted`. -/
theorem capability_bound_any_rule {P : Platform} {g : GrantRule} {s : State}
    (hr : Reachable P g s) {p : Pid} {c : Capsule} (hc : s.caps p = some c) :
    Within c.held (c.admitted ||| c.received) ∧
    Within c.received P.imageCeiling ∧
    Within c.held P.imageCeiling ∧
    Within c.admitted c.request.cert.ceiling ∧
    Within c.admitted P.imageCeiling := by
  obtain ⟨_, h2, h3, h4, h5⟩ := capInv_reachable s hr p c hc
  exact ⟨h2, h4, within_trans h2 (within_or h3 h4), admission_within_cert h5, h3⟩

/-- Under either rule, with no Admin in the image nothing is ever granted, so
    a capsule holds only its admitted word. -/
theorem capability_monotonicity_without_admin {P : Platform} {g : GrantRule} (hna : NoAdmin P)
    {s : State} (hr : Reachable P g s) {p : Pid} {c : Capsule} (hc : s.caps p = some c) :
    Within c.held c.admitted := by
  have h := (capability_bound_any_rule hr hc).1
  rw [nothingReceived_reachable hna s hr p c hc, Nat.or_zero] at h
  exact h

/-! ### 2. Admission soundness -/

/-- `sigOk` unfolded: a signature of the algorithm, under an unrevoked key the
    certificate lists for that algorithm, that the primitive accepts. -/
theorem sigOk_spec {P : Platform} {m : Manifest} {c : Cert} {alg : Nat}
    (h : sigOk P m c alg = true) :
    ∃ sg ∈ m.signatures, sg.alg = alg ∧ P.keyRevoked sg.keyId = false ∧
      ∃ k, c.keys.find? (·.keyId == sg.keyId) = some k ∧ k.alg = alg ∧
        P.sigVerify alg k.key m.signedRegion sg.sig = true := by
  unfold sigOk at h
  obtain ⟨sg, hm, hs⟩ := List.any_eq_true.1 h
  simp only [Bool.and_eq_true, beq_iff_eq, Bool.not_eq_true'] at hs
  obtain ⟨⟨ha, hrv⟩, hk⟩ := hs
  refine ⟨sg, hm, ha, hrv, ?_⟩
  revert hk
  cases hf : c.keys.find? (·.keyId == sg.keyId) with
  | none => intro hk; cases hk
  | some k =>
      intro hk
      simp only [Bool.and_eq_true, beq_iff_eq] at hk
      exact ⟨k, rfl, hk.1, hk.2⟩

/-- The production policy is Ed25519 (`AlgId` 1) and ML-DSA-65 (`AlgId` 3). -/
theorem requiredAlgs_eq : requiredAlgs = [1, 3] := rfl

/-- **Admission soundness.** Every capsule live in a reachable state was
    admitted on a request whose manifest asks for nothing above the
    publisher's certificate ceiling, whose grant stays inside the manifest,
    whose install word is under the image ceiling, that carries a verifying
    Ed25519 signature and a verifying ML-DSA-65 signature under unrevoked
    certificate keys, that passed the structural checks and the attestation
    gate; and the word on its PCB is that install word trimmed by the boot
    profile. -/
theorem admission_soundness {P : Platform} {g : GrantRule} {s : State} (hr : Reachable P g s)
    {p : Pid} {c : Capsule} (hc : s.caps p = some c) :
    let r := c.request
    r.structural = true ∧
    Within (r.manifest.required ||| r.manifest.optional) r.cert.ceiling ∧
    sigOk P r.manifest r.cert 1 = true ∧
    sigOk P r.manifest r.cert 3 = true ∧
    Within r.granted (r.manifest.required ||| r.manifest.optional) ∧
    Within r.install P.imageCeiling ∧
    r.attested = true ∧
    c.admitted = r.install &&& P.profileKeep := by
  obtain ⟨_, _, _, _, h5⟩ := capInv_reachable s hr p c hc
  obtain ⟨a1, a2, a3, a4, a5, a6, a7⟩ := admission_spec h5
  have hall := List.all_eq_true.1 a3
  rw [requiredAlgs_eq] at hall
  exact ⟨a1, a2, hall 1 (by simp), hall 3 (by simp), a4, a5, a6, a7⟩

/-! ### 3. Syscall-gate completeness -/

/-- **Syscall-gate completeness.** In every reachable state, a handler runs
    for capsule `p` and syscall `nr` only if `p`'s token is its own and
    current and the token's allowlist (`allowlist::allowed`) permits `nr`.
    That list is the one derived from the word `p` holds, so `nr` is in the
    ABI and the generated `cap_table` passes on that word; and the handler's
    effect belongs to `nr`. -/
theorem syscall_gate_completeness {P : Platform} {g : GrantRule} {s s' : State} (hr : Reachable P g s)
    {p : Pid} {nr : Nat} {h : Handler} (st : Step P g s (.call p nr h) s') :
    ∃ c, s.caps p = some c ∧ allowed c.token.list nr (P.launcher p) = true ∧
      inAbi nr = true ∧ c.token.subject = p ∧ c.epoch ≤ c.token.epoch ∧
      capTableAllows c.held (P.launcher p) nr = true ∧ h.runsUnder nr = true := by
  cases st with
  | call hg hrun _ =>
      obtain ⟨c, hc, _, hl⟩ := gate_some hg
      obtain ⟨c', hc', h1, h2, h3, h4⟩ := gate_on_held hr hg
      rw [hc] at hc'; cases hc'
      exact ⟨c, hc, hl, h1, h2, h3, h4, hrun⟩

/-- The same, read as denial: outside the allowlist the capsule's word
    yields, no handler is reached. The call is refused: the state is
    unchanged (EPERM, Linux personality) or the caller is torn down (a native
    capsule is killed); no other outcome exists. -/
theorem syscall_outside_allowlist_denied {P : Platform} {g : GrantRule} {s : State} (hr : Reachable P g s)
    {p : Pid} {nr : Nat} {c : Capsule} (hc : s.caps p = some c)
    (hout : capTableAllows c.held (P.launcher p) nr = false) :
    (∀ h s', ¬ Step P g s (.call p nr h) s') ∧
    (∀ s', Step P g s (.deny p nr) s' → s' = s ∨ s' = s.teardown p) ∧
    Step P g s (.deny p nr) s ∧ Step P g s (.deny p nr) (s.teardown p) := by
  have hg : gate P s p nr = false := by
    cases hgt : gate P s p nr with
    | false => rfl
    | true =>
        obtain ⟨c', hc', _, _, _, h4⟩ := gate_on_held hr hgt
        rw [hc] at hc'; cases hc'
        rw [h4] at hout; cases hout
  refine ⟨fun h s' st => ?_, fun s' st => ?_, Step.deny hg, Step.denyKill hg hc⟩
  · obtain ⟨c', hc', _, _, _, _, h4, _⟩ := syscall_gate_completeness hr st
    rw [hc] at hc'; cases hc'
    rw [h4] at hout; cases hout
  · cases st with
    | deny => exact Or.inl rfl
    | denyKill => exact Or.inr rfl

/-- Against the manifest: under the shipped grant rule, a syscall other than
    MkAttestDoc is reached only if the table passes on the word the
    capsule's manifest declared under the image ceiling, the manifest-derived
    allowlist. MkAttestDoc is the one requirement with a negation
    (`only_attest_doc_negates`): it refuses a holder of Network, so a capsule
    whose profile trimmed Network may reach it although its declared word
    would not. -/
theorem syscall_within_manifest {P : Platform} {s s' : State}
    (hr : Reachable P .targetEnvelope s) {p : Pid} {nr : Nat} {h : Handler}
    (st : Step P .targetEnvelope s (.call p nr h) s') (hmono : nr ≠ Data.sysMkAttestDoc) :
    ∃ c, s.caps p = some c ∧ capTableAllows (declared P c.request) (P.launcher p) nr = true := by
  obtain ⟨c, hc, _, _, _, _, h3, _⟩ := syscall_gate_completeness hr st
  refine ⟨c, hc, ?_⟩
  have hw := (capability_monotonicity hr hc).1
  unfold capTableAllows lookupReq at h3 ⊢
  cases hf : Data.capTable.find? (·.1 == nr) with
  | none => rw [hf] at h3; cases h3
  | some e =>
      rw [hf] at h3
      simp only [option_map_some] at h3 ⊢
      have hmem := List.mem_of_find?_eq_some hf
      have he : e.1 = nr := by simpa using List.find?_some hf
      have hn : e.2.hasNot = false := by
        cases hneg : e.2.hasNot with
        | false => rfl
        | true => exact absurd (he ▸ only_attest_doc_negates e hmem hneg) hmono
      exact Req.eval_mono hw _ e.2 hn h3

/-! ### 4. W^X preservation -/

/-- **W^X preservation.** W^X holds at boot, every step preserves it, and so
    no reachable state maps a page writable and executable. -/
theorem wx_preservation (P : Platform) {g : GrantRule} :
    WX State.init ∧ (∀ s op s', WX s → Step P g s op s' → WX s') ∧
      ∀ s, Reachable P g s → WX s :=
  ⟨wx_init, fun _ _ _ hs st => wx_step hs st, wx_reachable⟩

/-! ### 5. Isolation preservation -/

/-- Which operations may change capsule `q`'s capability entry. -/
def Op.touchesCaps (op : Op) (q : Pid) : Prop :=
  match op with
  | .spawn p _ => p = q
  | .quarantine p _ => p = q
  | .teardown p => p = q
  | .deny p _ => p = q
  | .call _ _ (.capGrant t _) => t = q
  | .call _ _ (.capRevoke t _) => t = q
  | _ => False

/-- Authority is per capsule: only an operation aimed at `q` changes `q`'s
    capabilities. IPC moves bytes only, and no mapping, surface or other
    capsule's step reaches another capsule's word. -/
theorem caps_change_only_by_target {P : Platform} {g : GrantRule} {s s' : State} {op : Op}
    (st : Step P g s op s') (q : Pid) (hch : s'.caps q ≠ s.caps q) : op.touchesCaps q := by
  cases st with
  | @spawn p _ _ _ _ =>
      by_cases he : q = p
      · exact he.symm
      · exact absurd (upd_other _ _ he) hch
  | @quarantine p _ _ _ =>
      by_cases he : q = p
      · exact he.symm
      · exact absurd (upd_other _ _ he) hch
  | @teardown p _ _ =>
      by_cases he : q = p
      · exact he.symm
      · exact absurd (upd_other _ _ he) hch
  | load => exact hch rfl
  | deny => exact absurd rfl hch
  | @denyKill p _ _ _ _ =>
      by_cases he : q = p
      · exact he.symm
      · exact absurd (upd_other _ _ he) hch
  | protect => exact hch rfl
  | surfaceFree => exact hch rfl
  | call _ _ eff =>
      cases eff with
      | @capGrant t _ _ _ _ _ _ _ =>
          by_cases he : q = t
          · exact he.symm
          · exact absurd (upd_other _ _ he) hch
      | @capRevoke t _ _ _ _ _ _ =>
          by_cases he : q = t
          · exact he.symm
          · exact absurd (upd_other _ _ he) hch
      | other => exact hch rfl
      | mmap => exact hch rfl
      | munmap => exact hch rfl
      | ipcSend => exact hch rfl
      | surfaceRegister => exact hch rfl
      | surfaceShare => exact hch rfl
      | surfaceAttach => exact hch rfl
      | surfaceRelease => exact hch rfl

/-- **Isolation preservation.** The isolation invariant holds at boot, every
    step (spawn, load, every syscall handler including grant, map, IPC and
    the surface calls, protect, quarantine, teardown, slot free) preserves it,
    so it holds in every reachable state. There, two distinct capsules share
    a frame only as views of one surface each of them owns or was named on by
    its owner; a private frame is in one address space at one address; and
    every capsule's token is bound to that capsule. -/
theorem isolation_preservation (P : Platform) {g : GrantRule} :
    Iso State.init ∧ (∀ s op s', Iso s → Step P g s op s' → Iso s') ∧
    ∀ s, Reachable P g s →
      Iso s ∧
      (∀ p q va va' m m', s.maps p va = some m → s.maps q va' = some m' →
        m.frame = m'.frame → p ≠ q →
        ∃ sid sf, s.frames m.frame = .surface sid ∧ s.surfaces sid = some sf ∧
          (p = sf.owner ∨ p ∈ sf.consented) ∧ (q = sf.owner ∨ q ∈ sf.consented)) ∧
      (∀ p va m, s.maps p va = some m → s.frames m.frame = .proc p →
        ∀ q va' m', s.maps q va' = some m' → m'.frame = m.frame → q = p ∧ va' = va) ∧
      (∀ p c, s.caps p = some c → c.token.subject = p) :=
  ⟨iso_init, fun _ _ _ hs st => iso_step hs st, fun s hr =>
    ⟨iso_reachable s hr,
     fun _ _ _ _ _ _ hp hq hf hne => shared_frame_is_consented hr hp hq hf hne,
     fun _ _ _ hp hpriv _ _ _ hq hf => private_frame_unshared hr hp hq hf.symm hpriv,
     fun p c hc => by rw [(capInv_reachable s hr p c hc).1]; rfl⟩⟩

end Formal
