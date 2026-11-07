/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Surface share rights (`src/kernel_core/surface_registry/share/rights_rule.rs`,
`rights_attach.rs`, `rule.rs`).

A surface's owner names the receivers that may attach it, each with a write
bit, at most `MAX_RECEIVERS` per surface. A process may attach a surface if it
is the project or holds a right for it. Teardown drops a process's rights; the project's release drops every right on the surface. The theorems state each of
those, on lists that mirror the kernel's `Vec<Right>` and the slot's
`[u32; NAMED_MAX]`.
-/

namespace Nonos.ShareRights

def maxReceivers : Nat := 4

structure Right where
  handle : Nat
  pid : Nat
  write : Bool
  deriving DecidableEq, Repr

inductive Grant where
  | new
  | already
  | full
  deriving DecidableEq, Repr

def countFor (rs : List Right) (h : Nat) : Nat := (rs.filter (·.handle == h)).length

/-- `grant`: an existing right gains the write bit; a new one is refused when
    the surface already has `MAX_RECEIVERS` receivers. -/
def grant (rs : List Right) (r : Right) : List Right × Grant :=
  if rs.any (fun x => x.handle == r.handle && x.pid == r.pid) then
    (rs.map fun x => if x.handle == r.handle && x.pid == r.pid then { x with write := x.write || r.write } else x,
     .already)
  else if countFor rs r.handle ≥ maxReceivers then (rs, .full)
  else (rs ++ [r], .new)

def revokeHandle (rs : List Right) (h : Nat) : List Right := rs.filter (·.handle != h)
def revokePid (rs : List Right) (p : Nat) : List Right := rs.filter (·.pid != p)

inductive Attach where
  | owner
  | receiver (write : Bool)
  | refused
  deriving DecidableEq, Repr

/-- `attach`, with `same` the thread-group comparison taken as equality. -/
def attach (rs : List Right) (owner : Option Nat) (h p : Nat) : Attach :=
  match owner with
  | none => .refused
  | some o =>
    if o = p then .owner
    else match rs.find? (fun r => r.handle == h && r.pid == p) with
      | some r => .receiver r.write
      | none => .refused

theorem revokePid_clears (rs : List Right) (p : Nat) : ∀ r ∈ revokePid rs p, r.pid ≠ p := by
  intro r hr
  have := (List.mem_filter.1 hr).2
  simpa using this

theorem revokePid_keeps (rs : List Right) (p : Nat) (r : Right) (hr : r ∈ rs) (hp : r.pid ≠ p) :
    r ∈ revokePid rs p :=
  List.mem_filter.2 ⟨hr, by simpa using hp⟩

theorem revokeHandle_clears (rs : List Right) (h : Nat) : ∀ r ∈ revokeHandle rs h, r.handle ≠ h := by
  intro r hr
  have := (List.mem_filter.1 hr).2
  simpa using this

/-- After teardown of `p`, `p` attaches nothing as a receiver. -/
theorem revoked_pid_not_receiver (rs : List Right) (o : Option Nat) (h p : Nat) (w : Bool) :
    attach (revokePid rs p) o h p ≠ .receiver w := by
  unfold attach
  cases o with
  | none => simp
  | some o =>
      simp only
      split
      · simp
      · cases hf : (revokePid rs p).find? (fun r => r.handle == h && r.pid == p) with
        | none => simp
        | some r =>
            have hm := List.mem_of_find?_eq_some hf
            have hpr := List.find?_some hf
            simp only [Bool.and_eq_true, beq_iff_eq] at hpr
            exact absurd hpr.2 (revokePid_clears rs p r hm)

/-- After the project releases `h`, nobody attaches it as a receiver. -/
theorem released_handle_no_receiver (rs : List Right) (o : Option Nat) (h p : Nat) (w : Bool) :
    attach (revokeHandle rs h) o h p ≠ .receiver w := by
  unfold attach
  cases o with
  | none => simp
  | some o =>
      simp only
      split
      · simp
      · cases hf : (revokeHandle rs h).find? (fun r => r.handle == h && r.pid == p) with
        | none => simp
        | some r =>
            have hm := List.mem_of_find?_eq_some hf
            have hpr := List.find?_some hf
            simp only [Bool.and_eq_true, beq_iff_eq] at hpr
            exact absurd hpr.1 (revokeHandle_clears rs h r hm)

/-- A receiver view comes from a right the project recorded. -/
theorem receiver_has_right (rs : List Right) (o : Option Nat) (h p : Nat) (w : Bool)
    (ha : attach rs o h p = .receiver w) : ∃ r ∈ rs, r.handle = h ∧ r.pid = p ∧ r.write = w := by
  unfold attach at ha
  cases o with
  | none => simp at ha
  | some o =>
      simp only at ha
      split at ha
      · simp at ha
      · cases hf : rs.find? (fun r => r.handle == h && r.pid == p) with
        | none => rw [hf] at ha; simp at ha
        | some r =>
            rw [hf] at ha
            simp only [Attach.receiver.injEq] at ha
            have hpr := List.find?_some hf
            simp only [Bool.and_eq_true, beq_iff_eq] at hpr
            exact ⟨r, List.mem_of_find?_eq_some hf, hpr.1, hpr.2, ha⟩

/-- A slot with no owner gives no view. -/
theorem no_owner_refused (rs : List Right) (h p : Nat) : attach rs none h p = .refused := rfl

/-- the project always attaches as owner. -/
theorem owner_attaches (rs : List Right) (h o : Nat) : attach rs (some o) h o = .owner := by
  simp [attach]

/-- A grant never takes a surface past `MAX_RECEIVERS` receivers. -/
theorem grant_bounded (rs : List Right) (r : Right) (h : countFor rs r.handle ≤ maxReceivers) :
    countFor (grant rs r).1 r.handle ≤ maxReceivers := by
  unfold grant
  split
  · unfold countFor
    rw [List.filter_map, List.length_map]
    have : (List.filter ((fun x => x.handle == r.handle) ∘ fun x =>
        if (x.handle == r.handle && x.pid == r.pid) = true then { x with write := x.write || r.write } else x) rs)
        = List.filter (fun x => x.handle == r.handle) rs := by
      apply List.filter_congr
      intro x _
      simp only [Function.comp]
      split <;> rfl
    rw [this]; exact h
  · split
    · exact h
    · rename_i _ hc
      unfold countFor at *
      rw [List.filter_append, List.length_append]
      simp only [List.filter_cons, beq_self_eq_true, if_true, List.filter_nil, List.length_cons,
        List.length_nil]
      omega

/-- A grant never lowers a write bit and never drops a right. -/
theorem grant_keeps (rs : List Right) (r x : Right) (hx : x ∈ rs) :
    ∃ y ∈ (grant rs r).1, y.handle = x.handle ∧ y.pid = x.pid ∧ (x.write = true → y.write = true) := by
  unfold grant
  split
  · refine ⟨if x.handle == r.handle && x.pid == r.pid then { x with write := x.write || r.write } else x,
      List.mem_map_of_mem _ hx, ?_, ?_, ?_⟩ <;> split <;> simp_all
  · split
    · exact ⟨x, hx, rfl, rfl, id⟩
    · exact ⟨x, List.mem_append_left _ hx, rfl, rfl, id⟩

/-! ### The slot's named groups (`rule.rs`) -/

def namedMax : Nat := 4

inductive Named where
  | added
  | already
  | full
  | refused
  deriving DecidableEq, Repr

structure Shares where
  owner : Nat
  named : List Nat

def Shares.name (s : Shares) (g : Nat) : Shares × Named :=
  if g = 0 then (s, .refused)
  else if g = s.owner ∨ g ∈ s.named then (s, .already)
  else if s.named.length < namedMax then ({ s with named := s.named ++ [g] }, .added)
  else (s, .full)

def Shares.mayAttach (s : Shares) (r : Nat) : Bool :=
  r != 0 && (r == s.owner || s.named.contains r)

theorem zero_never_attaches (s : Shares) : s.mayAttach 0 = false := by
  simp [Shares.mayAttach]

theorem attach_named_or_owner (s : Shares) (r : Nat) (h : s.mayAttach r = true) :
    r ≠ 0 ∧ (r = s.owner ∨ r ∈ s.named) := by
  simp only [Shares.mayAttach, Bool.and_eq_true, bne_iff_ne, Bool.or_eq_true, beq_iff_eq,
    List.contains_iff_mem] at h
  exact h

theorem name_zero_refused (s : Shares) : (s.name 0).2 = .refused := by simp [Shares.name]

theorem name_bounded (s : Shares) (g : Nat) (h : s.named.length ≤ namedMax) :
    (s.name g).1.named.length ≤ namedMax := by
  unfold Shares.name
  split
  · exact h
  · split
    · exact h
    · split
      · rename_i hl; simp only [List.length_append, List.length_cons, List.length_nil]; omega
      · exact h

/-- Naming only ever adds the group named, and only if it is not zero. -/
theorem name_adds_only (s : Shares) (g x : Nat) (hx : x ∈ (s.name g).1.named) :
    x ∈ s.named ∨ (x = g ∧ g ≠ 0) := by
  unfold Shares.name at hx
  split at hx
  · exact Or.inl hx
  · split at hx
    · exact Or.inl hx
    · split at hx
      · rename_i h0 _ _
        rcases List.mem_append.1 hx with h | h
        · exact Or.inl h
        · simp at h; exact Or.inr ⟨h, h0⟩
      · exact Or.inl hx

end Nonos.ShareRights
