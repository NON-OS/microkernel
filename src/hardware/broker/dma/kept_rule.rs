// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! A DMA run kept until its device's wipe has run (L78). An unconfined grant
//! of a PCI function, drained by a release, could still be reached by the
//! device after Bus Master off, so its frames wait for the wipe. A run the
//! surface registry also holds goes free once both have let go, never twice.

/// Whether a drained grant's frames are kept: a PCI function's, unconfined,
/// drained by a release (an unmap by its holder during the claim is the
/// holder's own word, as before).
pub(crate) const fn keeps(pci: bool, confined: bool, released: bool) -> bool {
    pci && !confined && released
}

/// A kept run: `let_go`, the registry no longer holds it; `settled`, its
/// wipe has ended and it may go.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) struct Run {
    pub(crate) let_go: bool,
    pub(crate) settled: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Act {
    /// Nothing yet: the other holder is still to let go.
    Wait,
    /// Scrub and free now.
    Free,
    /// Never: the device may still reach it.
    Keep,
}

/// The registry let go of the run.
pub(crate) const fn on_let_go(r: Run) -> (Run, Act) {
    let r = Run { let_go: true, settled: r.settled };
    (r, if r.settled { Act::Free } else { Act::Wait })
}

/// The wipe ended; `frees` from `end_rule::frees`.
pub(crate) const fn on_settle(r: Run, frees: bool) -> (Run, Act) {
    if !frees {
        return (r, Act::Keep);
    }
    let r = Run { let_go: r.let_go, settled: true };
    (r, if r.let_go { Act::Free } else { Act::Wait })
}
