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
//! Domains a device may still reach through a device table entry no unit
//! confirmed dropping: an attach whose flush failed, and whose rollback to
//! denied failed too. Their tables are never freed, since the unit may still
//! hold the entry naming them; their ids are never reused.

use core::sync::atomic::{AtomicBool, Ordering};

use super::lifetime::MAX_DOMAINS;

static PINNED: [AtomicBool; MAX_DOMAINS] = [const { AtomicBool::new(false) }; MAX_DOMAINS];

pub(super) fn pin(id: u16) {
    if let Some(p) = PINNED.get(id as usize) {
        p.store(true, Ordering::Release);
    }
}

pub(super) fn pinned(id: u16) -> bool {
    PINNED.get(id as usize).is_some_and(|p| p.load(Ordering::Acquire))
}
