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

//! Whether a claim may begin while a wipe record names the function (L78).
//! A wipe under way says Resetting, before any entry exists, so the claimant
//! waits in user code; a function no reset could be trusted on goes only to
//! its last holder's own service; one that did not come back is refused for
//! its hold and then asked once more.

use super::phase_rule::{Phase, LOST_HOLD};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Admit {
    Go,
    Rearm,
    Resetting,
    Refused,
}

/// `same`: the claimant serves the record's own driver service; `over_cap`:
/// the frames kept for good have reached their bound, which stops a new
/// tenancy only: a wipe under way still says Resetting, and a lost one is
/// still asked again, since its end may free what it kept.
pub(crate) const fn admits(record: Option<Phase>, same: bool, now: u64, over_cap: bool) -> Admit {
    let go = if over_cap { Admit::Refused } else { Admit::Go };
    match record {
        None => go,
        Some(Phase::Kept) if same => go,
        Some(Phase::Kept) => Admit::Refused,
        Some(Phase::Lost { at }) if now.saturating_sub(at) < LOST_HOLD || now < at => {
            Admit::Refused
        }
        Some(Phase::Lost { .. }) => Admit::Rearm,
        Some(_) => Admit::Resetting,
    }
}
