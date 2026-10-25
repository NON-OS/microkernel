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

//! Whether a wipe writes its reset now (L78). A function outside D0, or one
//! with no method or no marker to read, is never reset, so never trusted
//! reset; one with transactions pending waits for them up to the bound.

use super::marker_rule::Marker;
use super::method_rule::Method;
use super::phase_rule::{Phase, TP_BOUND};

/// What the start step read of the function.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) struct Look {
    pub(crate) in_d0: bool,
    pub(crate) tp: bool,
    pub(crate) marker: Option<Marker>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Do {
    Stay,
    Write(Method, Marker),
    Unreset,
}

/// `since`: when the reset was owed, from which pending transactions are
/// waited for.
pub(crate) const fn start(m: Option<Method>, look: Look, since: u64, now: u64) -> Do {
    let (Some(m), Some(mark)) = (m, look.marker) else { return Do::Unreset };
    if !look.in_d0 {
        return Do::Unreset;
    }
    if look.tp && now.saturating_sub(since) < TP_BOUND {
        return Do::Stay;
    }
    Do::Write(m, mark)
}

/// The phase a reset written at `now` leaves.
pub(crate) const fn written(m: Method, now: u64) -> Phase {
    match m {
        Method::Flr | Method::AfFlr => Phase::Flr { at: now },
        Method::PmCycle => Phase::D3 { at: now },
    }
}
