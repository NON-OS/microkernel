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

//! An installed disk's answer, from the bytes the checks read: the table
//! first, since nothing else on a disk this writer did not lay out means
//! what it would here, then where the partitions are, then the plan, and
//! the key header last, which is the one a person can change.

use super::key::{key_state, Key};
use super::places::placed;
use super::table::read_table;
use super::volume::plan_inside;
use super::why::NotKept;
use crate::guid::Guid;
use crate::layout::Layout;

/// An installed NØNOS an update can be laid over: where everything is, as
/// its own table says, and the identifiers that table gives it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Installed {
    pub layout: Layout,
    pub disk: Guid,
    /// In `Region::ALL` order: store, plan, data, ESP.
    pub partitions: [Guid; 4],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Keep {
    /// The new release opens the data volume, so an update keeps it.
    Ready(Installed),
    /// It would once the installed system prepares it, and not before.
    NeedsPrepare(Installed),
    /// An update would not keep it, and why.
    No(NotKept),
}

/// `head` is sectors 0 to 33 of a disk of `total` sectors; `plan` and
/// `key` the disk plan's sector and the key header's.
pub fn classify(total: u64, head: &[u8], plan: &[u8], key: &[u8]) -> Keep {
    let installed = match installed(total, head, plan) {
        Ok(i) => i,
        Err(why) => return Keep::No(why),
    };
    match key_state(key) {
        Ok(Key::Opens) => Keep::Ready(installed),
        Ok(Key::Unprepared) => Keep::NeedsPrepare(installed),
        Err(why) => Keep::No(why),
    }
}

fn installed(total: u64, head: &[u8], plan: &[u8]) -> Result<Installed, NotKept> {
    let table = read_table(total, head)?;
    let layout = placed(total, &table.extents)?;
    plan_inside(plan, &layout.data)?;
    Ok(Installed { layout, disk: table.disk, partitions: table.unique })
}
