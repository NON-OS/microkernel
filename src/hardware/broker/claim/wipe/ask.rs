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

//! The three questions the claims ask of the wipe records, each under the
//! claims' lock (L78): a claim's entry gone, so its wipe is owed; whether a
//! new claim may begin; and a lost function asked once more.

use core::sync::atomic::Ordering;

use super::admit_rule::{admits as rule, Admit};
use super::holder_rule::{same, Name};
use super::phase_rule::Phase;
use super::table::{PENDING, WIPES};

/// In the same hold that removes the claim's entry, so no claim can begin
/// between the entry's going and its wipe being owed.
pub(in crate::hardware::broker) fn owe(device_id: u64, epoch: u64) {
    let mut all = WIPES.lock();
    let key = |w: &&mut super::table::Wipe| w.device_id == device_id && w.epoch == epoch;
    if let Some(w) = all.iter_mut().find(key).filter(|w| w.phase == Phase::Held) {
        w.phase = Phase::Owed { since: crate::time::now_ns() };
        PENDING.fetch_add(1, Ordering::AcqRel);
    }
}

/// `over_cap` is asked once the record is read: an end settles its frames
/// before it is written, so an end read here has them counted (L80).
pub(in crate::hardware::broker) fn admits(
    device_id: u64,
    by: Option<Name>,
    over_cap: impl FnOnce() -> bool,
) -> Admit {
    let all = WIPES.lock();
    let record = all.iter().find(|w| w.device_id == device_id);
    let (now, capped) = (crate::time::now_ns(), over_cap());
    rule(record.map(|w| w.phase), same(record.and_then(|w| w.by), by), now, capped)
}

pub(in crate::hardware::broker) fn rearm(device_id: u64) {
    let mut all = WIPES.lock();
    let lost = |w: &&mut super::table::Wipe| {
        w.device_id == device_id && matches!(w.phase, Phase::Lost { .. })
    };
    if let Some(w) = all.iter_mut().find(lost) {
        w.phase = Phase::Recheck;
        PENDING.fetch_add(1, Ordering::AcqRel);
    }
}
