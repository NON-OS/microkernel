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

use crate::kernel_core::surface_registry::attach_map::snapshot::records_of;
use crate::smp::lock_responsive;

use super::frames_of::frames_of;
use super::gate::GATE;
use super::kept::keep;

/*
 * `group`'s last live thread has ended; the tables `tables` names will be
 * freed frame by frame (L36). Under the gate, its surfaces first refuse every
 * attach until their slots are released, so no attach lands after the
 * holders are counted. Then every surface it owns is emptied, so a later
 * attach finds nothing. One another process still has attached becomes an
 * orphan `reclaim` frees after its last holder (`exit_owned`); frames it
 * attached from another owner are that owner's. Both are kept from the
 * teardown. A second pass, from finalize, finds nothing new.
 */
pub fn keep_at_exit(group: u32, tables: u64) {
    let _gate = lock_responsive(&GATE);
    super::closing::close_owned(group);
    let mut kept = super::exit_owned::orphan_owned(group, tables);
    for (handle, _) in records_of(group) {
        match frames_of(handle) {
            Some((owner, frames)) if owner != group => kept.extend_from_slice(&frames),
            _ => {}
        }
    }
    keep(tables, kept);
}
