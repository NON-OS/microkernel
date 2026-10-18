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

//! The logical unit served on a bound device, and its size.

use super::probe::Probe;
use super::report::Stage;
use crate::disk::{capacity, max_lun, unit_ready, Disk, Pause};
use crate::state::State;

/// The first logical unit with a medium ready is served: on a card reader,
/// the slot with the card in it. An empty slot is given up at once.
pub(super) fn serve_first_ready(mut disk: Disk, state: &mut State, pause: Pause) -> Probe {
    let last = max_lun(&disk);
    let mut ready = Err(0);
    for lun in 0..=last {
        disk.lun = lun;
        ready = unit_ready(&disk, state, pause);
        if ready.is_ok() {
            break;
        }
    }
    if let Err(e) = ready {
        return Probe::Failed(Stage::NotReady, e);
    }
    let (blocks, block_len) = match capacity(&disk, state) {
        Ok(c) => c,
        Err(e) => return Probe::Failed(Stage::Capacity, e),
    };
    (disk.blocks, disk.block_len) = (blocks, block_len);
    Probe::Ours(disk)
}
