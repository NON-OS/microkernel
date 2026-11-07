/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

The kernel state the whole-system theorems range over: the capsules and their
capability words and tokens, each capsule's page mappings, who owns every
physical frame, the surface registry, and the IPC inboxes. Each field names
the structure in `src/` it stands for.
-/

import Formal.Bits

namespace Formal

abbrev Pid := Nat
abbrev Frame := Nat
abbrev VA := Nat

/-- Page permissions (`memory::paging::types::PagePermissions`). -/
structure Perm where
  read : Bool
  write : Bool
  exec : Bool
  user : Bool
  deriving DecidableEq, Repr

/-- `PagePermissions::is_wx_violation`: writable and executable at once. -/
def Perm.wx (p : Perm) : Bool := p.write && p.exec

/-- One leaf mapping in a capsule's address space. -/
structure Mapping where
  frame : Frame
  perm : Perm
  deriving DecidableEq, Repr

/-- The capability token the resolver reads (`capabilities::CapabilityToken`):
    who it is bound to, the revocation epoch it was minted under, its
    permission word, and the syscall allowlist derived from that word when it
    was installed (`syscall::contract::allowlist`). Signature, boot session
    and ASID are bound at mint time by `process::caps::new_token`; the model
    keeps the subject and the epoch, the fields a stale or foreign token
    would differ in. -/
structure Token where
  subject : Pid
  epoch : Nat
  perms : Caps
  /-- `Allowlist`: for a syscall number and the caller's launcher standing,
      whether the call is permitted. -/
  list : Nat → Bool → Bool

/-- A publisher signature carried in the manifest. -/
structure Signature where
  alg : Nat
  keyId : Nat
  sig : Nat
  deriving DecidableEq, Repr

/-- A publisher key in the NØNOS ID certificate. -/
structure PubKey where
  keyId : Nat
  alg : Nat
  key : Nat
  deriving DecidableEq, Repr

/-- The verified NØNOS ID (`VerifiedNonosId`): the publisher's capability
    ceiling and its keys. -/
structure Cert where
  ceiling : Caps
  keys : List PubKey
  deriving DecidableEq, Repr

/-- The capsule manifest fields admission reads (`CapsuleManifest`). -/
structure Manifest where
  required : Caps
  optional : Caps
  signatures : List Signature
  /-- The signed region (`signed_region::compute`), as an opaque digest. -/
  signedRegion : Nat
  deriving DecidableEq, Repr

/-- Everything `capsule_spawn::runner::preflight::run` is handed for one spawn. -/
structure Request where
  manifest : Manifest
  cert : Cert
  /-- `spec.requested_caps`, the grant the spawner asks for. -/
  granted : Caps
  /-- The checks the model takes as pass/fail inputs: manifest decode, ID
      certificate verification, certificate binding, namespace, payload hash,
      target triple and endpoint drift. -/
  structural : Bool
  /-- The attestation gate for the capsule's tier (`attest_gate` or
      `publisher_gate`). -/
  attested : Bool
  deriving DecidableEq, Repr

/-- A live capsule: its process control block's authority. -/
structure Capsule where
  /-- The spawn request it was admitted on, kept as admission evidence. -/
  request : Request
  /-- The word `install_caps` put on the PCB at admission. -/
  admitted : Caps
  /-- `pcb.caps_envelope`: the most a grant may give, recorded once at
      spawn. -/
  envelope : Caps
  /-- `pcb.caps_bits` now. -/
  held : Caps
  /-- Ghost: every bit `sys_cap_grant` has ever added to this capsule. -/
  received : Caps
  /-- `pcb.revocation_epoch`. -/
  epoch : Nat
  /-- `pcb.capability_token`. -/
  token : Token

/-- Who a physical frame belongs to. -/
inductive FrameOwner where
  | free
  /-- Private memory of one capsule (anonymous mmap, ELF segments, stacks). -/
  | proc (p : Pid)
  /-- A surface buffer, held by the registry slot rather than a process. -/
  | surface (sid : Nat)
  deriving DecidableEq, Repr

/-- A surface registry slot (`kernel_core::surface_registry`). -/
structure Surface where
  owner : Pid
  frames : List Frame
  /-- Live share rights: receiver and whether its view is writable. -/
  named : List (Pid × Bool)
  /-- Ghost: every receiver the project has ever named. -/
  consented : List Pid
  /-- Processes holding a view (the attach map). -/
  attached : List Pid
  /-- the project released the surface or exited; no new view is given. -/
  released : Bool
  deriving DecidableEq, Repr

structure State where
  caps : Pid → Option Capsule
  maps : Pid → VA → Option Mapping
  frames : Frame → FrameOwner
  surfaces : Nat → Option Surface
  inbox : Pid → List (List Nat)

/-- Boot, before any capsule: nothing admitted, nothing mapped, every frame
    free, an empty registry. -/
def State.init : State where
  caps := fun _ => none
  maps := fun _ _ => none
  frames := fun _ => .free
  surfaces := fun _ => none
  inbox := fun _ => []

/-- Point update. -/
def upd {α β : Type} [DecidableEq α] (f : α → β) (a : α) (b : β) : α → β :=
  fun x => if x = a then b else f x

@[simp] theorem upd_same {α β : Type} [DecidableEq α] (f : α → β) (a : α) (b : β) :
    upd f a b a = b := by simp [upd]

@[simp] theorem upd_other {α β : Type} [DecidableEq α] (f : α → β) {a x : α} (b : β)
    (h : x ≠ a) : upd f a b x = f x := by simp [upd, h]

theorem upd_apply {α β : Type} [DecidableEq α] (f : α → β) (a : α) (b : β) (x : α) :
    upd f a b x = if x = a then b else f x := rfl

end Formal
