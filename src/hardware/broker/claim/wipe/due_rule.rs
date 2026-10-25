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

//! When a wipe's next step is due (L78): each window in `phase_rule` is a
//! deadline a later step checks; a clock read behind the phase's own start,
//! as one CPU's can be against another's, reads as not yet.

use super::phase_rule::{Phase, FLR_WAIT, PM_WAIT, RETRY_WINDOW};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Due {
    Wait,
    Start,
    ToD0,
    Check,
}

const fn past(now: u64, at: u64, wait: u64) -> bool {
    now.saturating_sub(at) >= wait && now >= at
}

pub(crate) const fn due(p: Phase, now: u64) -> Due {
    match p {
        Phase::Held | Phase::Kept | Phase::Lost { .. } => Due::Wait,
        Phase::Owed { .. } => Due::Start,
        Phase::Flr { at } if past(now, at, FLR_WAIT) => Due::Check,
        Phase::D3 { at } if past(now, at, PM_WAIT) => Due::ToD0,
        Phase::D0 { at } if past(now, at, PM_WAIT) => Due::Check,
        Phase::Recheck => Due::Check,
        _ => Due::Wait,
    }
}

/// When a written reset must have come back, if it has a deadline.
pub(crate) const fn deadline(p: Phase) -> Option<u64> {
    match p {
        Phase::Flr { at } => Some(at.saturating_add(FLR_WAIT + RETRY_WINDOW)),
        Phase::D0 { at } => Some(at.saturating_add(PM_WAIT + RETRY_WINDOW)),
        _ => None,
    }
}

/// The phases a step moves on: the rest wait for a claim or a release.
pub(crate) const fn pending(p: Phase) -> bool {
    !matches!(p, Phase::Held | Phase::Kept | Phase::Lost { .. })
}
