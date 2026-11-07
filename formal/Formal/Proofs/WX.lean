/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

W^X: no page in any capsule's address space is writable and executable at
once. Every path that writes a leaf refuses a W+X permission
(`map_page`, `map_page_in_asid`, `update_page_flags`, the ELF segment check,
`AddressSpace::map_page` through `ProtectionFlags::is_wx_violation`), and a
surface view is never executable.
-/

import Formal.Proofs.Maps

namespace Formal

/-- The invariant. -/
def WX (s : State) : Prop :=
  ∀ p va m, s.maps p va = some m → m.perm.wx = false

theorem wx_init : WX State.init := by
  intro p va m h
  simp [State.init] at h

theorem wx_step {P : Platform} {g : GrantRule} {s s' : State} {op : Op} (hs : WX s) (st : Step P g s op s') :
    WX s' := by
  cases st with
  | spawn => exact hs
  | load _ _ _ hw =>
      intro q va m h
      rcases mapAt_some h with ⟨_, _, he⟩ | ⟨_, h⟩
      · cases he; exact hw
      · exact hs _ _ _ h
  | deny => exact hs
  | denyKill =>
      intro q va m h
      exact hs _ _ _ (teardown_maps_some h).2
  | protect _ hw =>
      intro q va m h
      rcases mapAt_some h with ⟨_, _, he⟩ | ⟨_, h⟩
      · cases he; exact hw
      · exact hs _ _ _ h
  | quarantine => exact hs
  | teardown =>
      intro q va m h
      exact hs _ _ _ (teardown_maps_some h).2
  | surfaceFree => exact hs
  | call _ _ eff =>
      cases eff with
      | other => exact hs
      | capGrant => exact hs
      | capRevoke => exact hs
      | mmap _ _ hw =>
          intro q va m h
          rcases mapAt_some h with ⟨_, _, he⟩ | ⟨_, h⟩
          · cases he; exact hw
          · exact hs _ _ _ h
      | munmap =>
          intro q va m h
          rcases mapAt_some h with ⟨_, _, he⟩ | ⟨_, h⟩
          · cases he
          · exact hs _ _ _ h
      | ipcSend => exact hs
      | surfaceRegister => exact hs
      | surfaceShare => exact hs
      | surfaceAttach =>
          intro q va m h
          rcases mapAt_some h with ⟨_, _, he⟩ | ⟨_, h⟩
          · cases he; simp [Perm.wx]
          · exact hs _ _ _ h
      | surfaceRelease =>
          intro q va m h
          exact hs _ _ _ (unmapSurface_some h).1

/-- **W^X preservation**: W^X holds at boot and after every step, so in every
    reachable state. -/
theorem wx_reachable {P : Platform} {g : GrantRule} : ∀ s, Reachable P g s → WX s :=
  Reachable.induct wx_init fun _ _ _ _ hs st => wx_step hs st

end Formal
