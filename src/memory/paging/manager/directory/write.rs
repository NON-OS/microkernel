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

//! Changing the directory. Only the paging manager lock's holder does, so
//! two writers never meet in a slot and a writer reads its own slots freely.

use core::sync::atomic::Ordering::*;

use super::slot::Entry;
use super::table::{FULL, HIGH, TABLE};

/// `asid` now names the tables at `root`, owned by `pid`.
pub(crate) fn publish(asid: u32, pid: u32, root: u64) {
    match TABLE.iter().position(|slot| slot.read().is_none()) {
        Some(i) => {
            TABLE[i].write(Some((asid, pid, root)));
            HIGH.fetch_max(i + 1, Release);
        }
        None => FULL.store(true, Release),
    }
}

/// `asid` names nothing any more.
pub(crate) fn withdraw(asid: u32) {
    if let Some(slot) = TABLE.iter().find(|slot| holds(slot.read(), asid)) {
        slot.write(None);
    }
}

/// `pid` now owns `asid`; false when the directory does not hold it.
pub(crate) fn hand_over(asid: u32, pid: u32) -> bool {
    for slot in TABLE.iter() {
        if let Some((held, _, root)) = slot.read().filter(|&e| holds(Some(e), asid)) {
            slot.write(Some((held, pid, root)));
            return true;
        }
    }
    false
}

fn holds(entry: Option<Entry>, asid: u32) -> bool {
    entry.is_some_and(|(held, _, _)| held == asid)
}
