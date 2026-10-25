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

//! The kernel as built in the takeover model, or one step of its design
//! undone for an undo check (L79). Each undone step is one the kernel
//! encodes as an order of calls, so the model is where it can be turned.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum Knob {
    Built,
    /// The takeover removed: no stop, no take-back loop, and the admit as it
    /// was before, which asks nothing of bootfb's claim and latches nothing.
    Control,
    /// GONE latched when the takeover begins, not at the admit.
    GoneEarly,
    /// A release of bootfb's claim under way read as done, in the loop and
    /// at the admit.
    LeavingDone,
    /// The takeover run after the admit instead of before it.
    TakeAfterAdmit,
    /// The take-back's release before its repoint.
    ReleaseFirst,
    /// MkMmioUnmap not counted inside its claim.
    UnmapOutside,
    /// The repoint without its check that a page still shows its frame.
    NoTranslateCheck,
    /// The wipe record written before the scanout check.
    ScanoutAfterWipe,
    /// TAKERS one flag for every takeover, cleared by whichever ends first.
    TakersFlag,
    /// The drawer reads the stop only before it counts its draw.
    NoRecheck,
    /// The hold left set as bootfb's entry goes, so neither its release nor
    /// its exit gives the console back.
    HoldKept,
}
