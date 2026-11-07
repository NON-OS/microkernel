/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

The step relation. A capsule changes the kernel only through a syscall, and a
syscall's handler runs only past the gate (`Op.call`); a refused call changes
nothing (`Op.deny`). The kernel's own transitions are admission, ELF and stack
loading during spawn, page protection changes, the quarantine revoke, process
teardown and freeing a surface slot. Each constructor names the Rust it
stands for.
-/

import Formal.Gate

namespace Formal

/-- The handler effects the model tracks. `other` is every handler whose
    effect is on state outside the model (time, futex, logging, device
    queries); it may run under any number and changes nothing here. -/
inductive Handler where
  | other
  /-- `sys_cap_grant(target, mask)`. -/
  | capGrant (target : Pid) (mask : Caps)
  /-- `sys_cap_revoke(target, mask)`. -/
  | capRevoke (target : Pid) (mask : Caps)
  /-- MkMmap: a fresh frame mapped into the caller. -/
  | mmap (va : VA) (f : Frame) (perm : Perm)
  /-- MkMunmap: the caller's own page unmapped and its frame freed. -/
  | munmap (va : VA)
  /-- An IPC send: bytes into the receiver's inbox. -/
  | ipcSend (dst : Pid) (msg : List Nat)
  /-- `surface_registry` register: fresh frames become a surface slot. -/
  | surfaceRegister (sid : Nat) (fs : List Frame)
  /-- `share_surface`: the project names a receiver. -/
  | surfaceShare (sid : Nat) (recv : Pid) (write : Bool)
  /-- `attach_surface`: a view of one surface frame in the caller. -/
  | surfaceAttach (sid : Nat) (va : VA) (f : Frame)
  /-- `release_surface` / `owner_releases`: the caller drops its views. -/
  | surfaceRelease (sid : Nat)

/-- The syscall numbers whose handler has this effect; `none` for any. -/
def Handler.syscalls : Handler → Option (List Nat)
  | .other => none
  | .capGrant .. => some [Data.sysMkCapGrant]
  | .capRevoke .. => some [Data.sysMkCapRevoke]
  | .mmap .. => some [Data.sysMkMmap]
  | .munmap _ => some [Data.sysMkMunmap]
  | .ipcSend .. => some [Data.sysMkIpcSend, Data.sysMkIpcSendToPid, Data.sysMkIpcCall,
      Data.sysMkIpcReply, Data.sysMkIpcReplyTo]
  | .surfaceRegister .. => some [Data.sysMkSurfaceRegister]
  | .surfaceShare .. => some [Data.sysMkSurfaceShare]
  | .surfaceAttach .. => some [Data.sysMkSurfaceAttach]
  | .surfaceRelease _ => some [Data.sysMkSurfaceRelease]

def Handler.runsUnder (h : Handler) (nr : Nat) : Bool :=
  match h.syscalls with
  | none => true
  | some l => l.contains nr

inductive Op where
  /-- `preflight::run` then `install::run` for a new process `p`. -/
  | spawn (p : Pid) (r : Request)
  /-- `load_elf_into_pid` and the stack allocations during spawn. -/
  | load (p : Pid) (va : VA) (f : Frame) (perm : Perm)
  /-- A syscall by `p` whose handler runs with effect `h`. -/
  | call (p : Pid) (nr : Nat) (h : Handler)
  /-- A syscall by `p` the gate refuses: EPERM, or the caller killed. -/
  | deny (p : Pid) (nr : Nat)
  /-- `PagingManager::update_page_flags`. -/
  | protect (p : Pid) (va : VA) (perm : Perm)
  /-- `process::core::isolation`: the kernel revokes dangerous bits. -/
  | quarantine (p : Pid) (mask : Caps)
  /-- `process::exit::teardown`: exit, kill, fault or failed spawn. -/
  | teardown (p : Pid)
  /-- The last reference to a released surface slot goes. -/
  | surfaceFree (sid : Nat)

/-- The grant bound `install_caps` records (`caps_envelope::record`): the
    manifest's required and optional bits under the image ceiling, trimmed by
    the boot profile, with the installed word. -/
def envelopeOf (P : Platform) (r : Request) (w : Caps) : Caps :=
  ((r.manifest.required ||| r.manifest.optional) &&& P.imageCeiling &&& P.profileKeep) ||| w

/-- The capsule `install_spawn` creates: the token is minted for `p` over the
    installed word at epoch 0. -/
def Capsule.fresh (P : Platform) (p : Pid) (r : Request) (w : Caps) : Capsule where
  request := r
  admitted := w
  envelope := envelopeOf P r w
  held := w
  received := 0
  epoch := 0
  token := mint p 0 w

/-- `process::caps::grant`: OR the mask in and mint a fresh token. -/
def Capsule.grant (t : Pid) (c : Capsule) (mask : Caps) : Capsule :=
  { c with held := c.held ||| mask, received := c.received ||| mask,
           token := mint t c.epoch (c.held ||| mask) }

/-- `process::caps::revoke`: bump the epoch, clear the mask, mint a fresh
    token under the new epoch. -/
def Capsule.revoke (t : Pid) (c : Capsule) (mask : Caps) : Capsule :=
  { c with held := excess c.held mask, epoch := c.epoch + 1,
           token := mint t (c.epoch + 1) (excess c.held mask) }

/-- Whether `p`'s view of a surface is writable: the project's always is, a
    receiver's as its right says. -/
def Surface.attachWrite (sf : Surface) (p : Pid) : Bool :=
  sf.owner == p || sf.named.any fun e => e.1 == p && e.2

/-- `share_surface`'s record: a right for the receiver, and the project's
    consent noted. -/
def Surface.share (sf : Surface) (q : Pid) (w : Bool) : Surface :=
  { sf with named := (q, w) :: sf.named, consented := q :: sf.consented }

/-- `p` takes a view. -/
def Surface.attach (sf : Surface) (p : Pid) : Surface :=
  { sf with attached := p :: sf.attached }

/-- `p` drops its views; the project's release also revokes every right. -/
def Surface.releaseBy (p : Pid) (sf : Surface) : Surface :=
  let s1 := { sf with attached := sf.attached.filter (· != p) }
  if sf.owner = p then { s1 with released := true, named := [] } else s1

/-- Teardown's effect on a slot: `rights::revoke_pid`, the views go, and an
    exiting owner releases. -/
def Surface.drop (p : Pid) (sf : Surface) : Surface :=
  let s1 := { sf with attached := sf.attached.filter (· != p),
                      named := sf.named.filter (·.1 != p) }
  if sf.owner = p then { s1 with released := true, named := [] } else s1

/-- `process::exit::teardown`: the PCB, the address space, the private frames,
    the surface views and rights, and the inbox go. -/
def State.teardown (s : State) (p : Pid) : State where
  caps := upd s.caps p none
  maps := upd s.maps p (fun _ => none)
  frames := fun f => if s.frames f = .proc p then .free else s.frames f
  surfaces := fun sid => (s.surfaces sid).map (Surface.drop p)
  inbox := upd s.inbox p []

/-- `p`'s mappings with `va` set to `m`. -/
def State.mapAt (s : State) (p : Pid) (va : VA) (m : Option Mapping) : Pid → VA → Option Mapping :=
  upd s.maps p (upd (s.maps p) va m)

/-- `p`'s mappings with every view of surface `sid` gone. -/
def State.unmapSurface (s : State) (p : Pid) (sid : Nat) : Pid → VA → Option Mapping :=
  upd s.maps p fun va =>
    match s.maps p va with
    | some m => if s.frames m.frame = .surface sid then none else some m
    | none => none

/-- Which grant rule the kernel runs. `targetEnvelope` is the shipped rule
    (`grant_rule::grant_decision`): the mask must also lie inside the
    target's envelope. `callerOnly` is the rule before that bound existed,
    kept so the counterexample it allowed stays a checked theorem. -/
inductive GrantRule where
  | callerOnly
  | targetEnvelope
  deriving DecidableEq, Repr

/-- The target-side condition the rule puts on a grant. -/
def GrantRule.allows : GrantRule → Caps → Capsule → Prop
  | .callerOnly, _, _ => True
  | .targetEnvelope, mask, ct => Within mask ct.envelope

/-- Init, the first process the kernel creates. -/
def initPid : Pid := 1

/-- `grant_rule::revoke_decision`. -/
def revokeDecision (same callerAdmin callerInit callerParent targetProtected : Bool) : Bool :=
  if same then true
  else if !callerAdmin then false
  else if targetProtected then callerInit
  else callerInit || callerParent

/-- What a handler does, given the caller `p` got past the gate. -/
inductive Effect (P : Platform) (g : GrantRule) (s : State) (p : Pid) : Handler → State → Prop
  | other : Effect P g s p .other s
  /-- Refused unless the caller holds Admin and every bit it grants, the
      target exists, and the rule's target bound holds. -/
  | capGrant {t : Pid} {mask : Caps} {cp ct : Capsule} :
      s.caps p = some cp → has cp.held Data.capAdmin = true → Within mask cp.held →
      s.caps t = some ct → g.allows mask ct →
      Effect P g s p (.capGrant t mask) { s with caps := upd s.caps t (some (ct.grant t mask)) }
  /-- `sys_cap_revoke`: self-revoke freely; another process only with Admin
      and as init or its parent; a protected service only from init. -/
  | capRevoke {t : Pid} {mask : Caps} {cp ct : Capsule} :
      s.caps p = some cp →
      revokeDecision (p == t) (has cp.held Data.capAdmin) (p == initPid) (p == P.parentOf t)
        (P.protectedSvc t) = true →
      s.caps t = some ct →
      Effect P g s p (.capRevoke t mask) { s with caps := upd s.caps t (some (ct.revoke t mask)) }
  /-- The frame comes off the free list; the permission passes `map_page`'s
      W^X refusal. -/
  | mmap {va : VA} {f : Frame} {perm : Perm} :
      s.frames f = .free → s.maps p va = none → perm.wx = false →
      Effect P g s p (.mmap va f perm)
        { s with maps := s.mapAt p va (some ⟨f, perm⟩), frames := upd s.frames f (.proc p) }
  | munmap {va : VA} {m : Mapping} :
      s.maps p va = some m → s.frames m.frame = .proc p →
      Effect P g s p (.munmap va)
        { s with maps := s.mapAt p va none, frames := upd s.frames m.frame .free }
  /-- IPC carries bytes (`IpcMessage { data: Vec<u8>, .. }`): no capability
      and no frame moves with a message. -/
  | ipcSend {dst : Pid} {msg : List Nat} {c : Capsule} :
      s.caps dst = some c →
      Effect P g s p (.ipcSend dst msg) { s with inbox := upd s.inbox dst (s.inbox dst ++ [msg]) }
  | surfaceRegister {sid : Nat} {fs : List Frame} :
      s.surfaces sid = none → (∀ f ∈ fs, s.frames f = .free) →
      Effect P g s p (.surfaceRegister sid fs)
        { s with surfaces := upd s.surfaces sid (some ⟨p, fs, [], [], [], false⟩),
                 frames := fun f => if f ∈ fs then .surface sid else s.frames f }
  /-- Only the project names, only a live process other than itself, only
      while the slot is not released. -/
  | surfaceShare {sid : Nat} {q : Pid} {w : Bool} {sf : Surface} {c : Capsule} :
      s.surfaces sid = some sf → sf.owner = p → sf.released = false → s.caps q = some c → q ≠ p →
      Effect P g s p (.surfaceShare sid q w)
        { s with surfaces := upd s.surfaces sid (some (sf.share q w)) }
  /-- `permit` and `rights::may_attach`: the project, or a receiver it named.
      The view is never executable. -/
  | surfaceAttach {sid : Nat} {va : VA} {f : Frame} {sf : Surface} :
      s.surfaces sid = some sf → sf.released = false → f ∈ sf.frames → s.maps p va = none →
      (sf.owner = p ∨ ∃ w, (p, w) ∈ sf.named) →
      Effect P g s p (.surfaceAttach sid va f)
        { s with maps := s.mapAt p va (some ⟨f, ⟨true, sf.attachWrite p, false, true⟩⟩),
                 surfaces := upd s.surfaces sid (some (sf.attach p)) }
  | surfaceRelease {sid : Nat} {sf : Surface} :
      s.surfaces sid = some sf →
      Effect P g s p (.surfaceRelease sid)
        { s with maps := s.unmapSurface p sid,
                 surfaces := upd s.surfaces sid (some (sf.releaseBy p)) }

inductive Step (P : Platform) (g : GrantRule) : State → Op → State → Prop
  | spawn {s : State} {p : Pid} {r : Request} {w : Caps} :
      s.caps p = none → admission P r = some w →
      Step P g s (.spawn p r) { s with caps := upd s.caps p (some (Capsule.fresh P p r w)) }
  /-- ELF segments pass `load_segment::validate`'s W^X refusal and
      `map_page`'s; stacks are read-write; `AddressSpace::map_page` refuses
      W+X too. -/
  | load {s : State} {p : Pid} {va : VA} {f : Frame} {perm : Perm} {c : Capsule} :
      s.caps p = some c → s.frames f = .free → s.maps p va = none → perm.wx = false →
      Step P g s (.load p va f perm)
        { s with maps := s.mapAt p va (some ⟨f, perm⟩), frames := upd s.frames f (.proc p) }
  | call {s s' : State} {p : Pid} {nr : Nat} {h : Handler} :
      gate P s p nr = true → h.runsUnder nr = true → Effect P g s p h s' →
      Step P g s (.call p nr h) s'
  /-- Outside the list: a Linux-personality process gets EPERM and nothing
      changes. -/
  | deny {s : State} {p : Pid} {nr : Nat} :
      gate P s p nr = false → Step P g s (.deny p nr) s
  /-- Outside the list: a native capsule is killed. -/
  | denyKill {s : State} {p : Pid} {nr : Nat} {c : Capsule} :
      gate P s p nr = false → s.caps p = some c → Step P g s (.deny p nr) (s.teardown p)
  | protect {s : State} {p : Pid} {va : VA} {perm : Perm} {m : Mapping} :
      s.maps p va = some m → perm.wx = false →
      Step P g s (.protect p va perm) { s with maps := s.mapAt p va (some ⟨m.frame, perm⟩) }
  | quarantine {s : State} {p : Pid} {mask : Caps} {c : Capsule} :
      s.caps p = some c →
      Step P g s (.quarantine p mask) { s with caps := upd s.caps p (some (c.revoke p mask)) }
  | teardown {s : State} {p : Pid} {c : Capsule} :
      s.caps p = some c → Step P g s (.teardown p) (s.teardown p)
  | surfaceFree {s : State} {sid : Nat} {sf : Surface} :
      s.surfaces sid = some sf → sf.released = true → sf.attached = [] →
      Step P g s (.surfaceFree sid)
        { s with surfaces := upd s.surfaces sid none,
                 frames := fun f => if s.frames f = .surface sid then .free else s.frames f }

/-- Every state the kernel can reach from boot. -/
inductive Reachable (P : Platform) (g : GrantRule) : State → Prop
  | init : Reachable P g State.init
  | step {s s' : State} {op : Op} : Reachable P g s → Step P g s op s' → Reachable P g s'

/-- An invariant that holds at boot and that every step preserves holds in
    every reachable state. -/
theorem Reachable.induct {P : Platform} {g : GrantRule} {I : State → Prop} (h0 : I State.init)
    (hs : ∀ s op s', Reachable P g s → I s → Step P g s op s' → I s') :
    ∀ s, Reachable P g s → I s := by
  intro s hr
  induction hr with
  | init => exact h0
  | step hr st ih => exact hs _ _ _ hr ih st

end Formal
