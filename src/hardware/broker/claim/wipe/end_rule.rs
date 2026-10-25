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

//! What a written reset came to (L78): Clean only when the function answers
//! as itself with its marker gone; back with it set, the one fallback, else
//! Unreset; not back by its deadline, Lost.

use super::due_rule::deadline;
use super::method_rule::Method;
use super::phase_rule::Phase;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum End {
    Clean,
    Unreset,
    Lost,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Next {
    Stay,
    Retry(Method),
    Ended(End),
}

/// `ready`: it answers as itself; `confirmed`: and its marker is gone.
pub(crate) const fn checked(
    p: Phase,
    now: u64,
    ready: bool,
    confirmed: bool,
    fallback: Option<Method>,
) -> Next {
    match (ready, confirmed, fallback, p, deadline(p)) {
        (true, true, ..) => Next::Ended(End::Clean),
        (true, false, Some(m), ..) => Next::Retry(m),
        (true, false, None, ..) => Next::Ended(End::Unreset),
        (false, _, _, Phase::Recheck, _) => Next::Ended(End::Lost),
        (false, _, _, _, Some(d)) if now >= d => Next::Ended(End::Lost),
        _ => Next::Stay,
    }
}

/// Whether kept frames go: after a reset unless the function reaches past
/// Bus Master (`binds`); unreset, only with Bus Master read back off too.
pub(crate) const fn frees(end: End, bme_off: bool, binds: bool) -> bool {
    match end {
        End::Clean => !binds,
        End::Unreset => bme_off && !binds,
        End::Lost => false,
    }
}

/// The record left: none after a clean reset, but a display function's
/// memory may outlive one, so it is kept for its own service as if unreset.
pub(crate) const fn ended(end: End, display: bool, now: u64) -> Option<Phase> {
    match end {
        End::Clean if !display => None,
        End::Clean | End::Unreset => Some(Phase::Kept),
        End::Lost => Some(Phase::Lost { at: now }),
    }
}
