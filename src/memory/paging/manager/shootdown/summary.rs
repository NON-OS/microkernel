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

//! What one round reached, kept only for `nonos-shootdown-trace`; a kernel
//! built without it keeps a unit and prints nothing.

/// What one round reached, for `trace`; a kernel built without it keeps none.
#[cfg(feature = "nonos-shootdown-trace")]
pub(super) struct Round {
    pub(super) asid: u32,
    pub(super) pages: u32,
    pub(super) selected: [u64; super::select::WORDS],
    pub(super) targets: u32,
    pub(super) starting: u32,
    pub(super) answered_for: u32,
}

#[cfg(not(feature = "nonos-shootdown-trace"))]
pub(super) type Round = ();

#[cfg(feature = "nonos-shootdown-trace")]
pub(super) fn summary(
    asid: u32,
    pages: u32,
    selected: [u64; super::select::WORDS],
    targets: u32,
    starting: u32,
    answered_for: u32,
) -> Round {
    Round { asid, pages, selected, targets, starting, answered_for }
}

#[cfg(not(feature = "nonos-shootdown-trace"))]
pub(super) fn summary(
    _: u32,
    _: u32,
    _: [u64; super::select::WORDS],
    _: u32,
    _: u32,
    _: u32,
) -> Round {
}

/// After the round, its lock released: the line takes the serial lock, and no
/// cpu may wait on this one for an acknowledgement while it does.
pub(super) fn after(done: Option<Round>) {
    #[cfg(feature = "nonos-shootdown-trace")]
    if let Some(done) = &done {
        super::trace::report(done);
    }
    #[cfg(not(feature = "nonos-shootdown-trace"))]
    let _ = done;
}
