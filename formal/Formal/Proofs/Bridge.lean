/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

The bridge to `verification/lean`'s `Nonos.SpawnCaps`, which proves the
spawn ceiling over capability sets (`Nat → Bool`). The model keeps words; the
two agree bit by bit, so the existing theorem is cited rather than proven
again.
-/

import Nonos.SpawnCaps
import Formal.Admission

namespace Formal

/-- A word as the set of its bits. -/
def toSet (c : Caps) : Nonos.SpawnCaps.Caps := fun i => has c i

theorem within_iff_set (a b : Caps) :
    Within a b ↔ Nonos.SpawnCaps.Within (toSet a) (toSet b) :=
  within_iff a b

theorem toSet_install (r o g : Caps) :
    toSet (installCaps r o g) =
      Nonos.SpawnCaps.installCaps (toSet r) (toSet o) (toSet g) := by
  funext i
  simp only [toSet, installCaps, Nonos.SpawnCaps.installCaps, Nonos.SpawnCaps.union,
    Nonos.SpawnCaps.inter, has_or, has_and]

theorem toSet_or (a b : Caps) :
    toSet (a ||| b) = Nonos.SpawnCaps.union (toSet a) (toSet b) := by
  funext i
  simp only [toSet, Nonos.SpawnCaps.union, has_or]

/-- `check_ceiling` passing bounds the installed word by the publisher's
    ceiling: `Nonos.SpawnCaps.installed_within_ceiling`, read on words. -/
theorem install_within_cert {r o g ceiling : Caps}
    (hc : Within (r ||| o) ceiling) : Within (installCaps r o g) ceiling := by
  rw [within_iff_set, toSet_install]
  apply Nonos.SpawnCaps.installed_within_ceiling
  unfold Nonos.SpawnCaps.withinCeiling
  rw [← toSet_or, ← within_iff_set]
  exact hc

end Formal
