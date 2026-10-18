/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Input authority admission (`src/capabilities/token/types/authority_broker.rs`
and the input gates in `src/syscall/contract/cap_table/mk_surface.rs`). Posting
an input event and consuming (draining) the global input ring are two separate
rights since K8: posting takes `InputSource`, held by the input drivers;
draining and waiting take `InputConsumer`, held by input_router alone. Neither
`Irq`, which every device driver holds, nor `Admin` stands in for either: a
driver that could drain could read the focused application's keystrokes, and
one that could post could forge them. The theorems fix each gate to its one
right, close both to a bare interrupt owner, and show the router drains
without being able to post.
-/

namespace Nonos.InputConsumer

/-- The input-relevant rights a capability token may hold. -/
structure Rights where
  inputSource : Bool
  inputConsumer : Bool
  irq : Bool
  admin : Bool

/-- `can_input_source`: may POST an input event. `InputSource` alone. -/
def canPost (r : Rights) : Bool := r.inputSource

/-- `can_input_consumer`: may DRAIN or WAIT on the input ring. `InputConsumer`
    alone. -/
def canDrain (r : Rights) : Bool := r.inputConsumer

/-- Posting is exactly `InputSource`: no other right, `Irq` and `Admin`
    included, changes the verdict. -/
theorem post_iff_source (r : Rights) : canPost r = r.inputSource := rfl

/-- Draining is exactly `InputConsumer`. -/
theorem drain_iff_consumer (r : Rights) : canDrain r = r.inputConsumer := rfl

/-- A driver holding `Irq` or `Admin` but neither input right may neither post
    nor drain: the keylogging and the key-forging paths are both closed. -/
theorem irq_only_cannot_drain (r : Rights)
    (h1 : r.inputSource = false) (h2 : r.inputConsumer = false) :
    canPost r = false ∧ canDrain r = false := ⟨h1, h2⟩

/-- The two rights are independent: input_router drains without posting. -/
theorem the_router_drains_without_posting :
    canDrain ⟨false, true, false, false⟩ = true ∧
      canPost ⟨false, true, false, false⟩ = false := ⟨rfl, rfl⟩

end Nonos.InputConsumer
