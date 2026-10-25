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

use core::sync::atomic::Ordering;

use super::pending::{Entry, SLOTS};

pub(super) fn find_by_intid(intid: u32) -> Option<&'static Entry> {
    if intid == 0 {
        return None;
    }
    SLOTS.iter().find(|e| e.intid.load(Ordering::Acquire) == intid)
}

pub(super) fn find_by_grant(grant_id: u64) -> Option<&'static Entry> {
    if grant_id == 0 {
        return None;
    }
    SLOTS.iter().find(|e| {
        e.intid.load(Ordering::Acquire) != 0 && e.grant_id.load(Ordering::Acquire) == grant_id
    })
}

// Mark the slot free. Caller must have already unregistered the GIC
// handler before this so no trampoline can race the slot's reuse.
pub(super) fn free(e: &'static Entry) {
    e.grant_id.store(0, Ordering::Release);
    e.intid.store(0, Ordering::Release);
}
