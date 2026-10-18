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

//! What a unit asked to its size is taken for. Before any device is served,
//! the first unit ready is ours. After, only the unit served is
//! (`identity`): another unit of the same device is passed for the next,
//! since a card reader's served card may sit in a later slot than a card put
//! in after it, and the whole reader was refused on its first slot. When no
//! unit is left the device is not taken.

use super::ask::Step;
use super::asking::Asking;
use super::identity::Identity;
use super::report::Stage;
use crate::disk::Disk;
use crate::protocol::E_NODEV;

impl Asking {
    pub(super) fn sized(self, (blocks, block_len): (u64, u32), served: Option<Identity>) -> Step {
        let disk = Disk { blocks, block_len, ..self.disk };
        let id = Identity::of(&disk, self.named, self.serial);
        match served {
            Some(s) if s != id => self.next_unit(Stage::Other, E_NODEV),
            _ => Step::Ours(disk, id),
        }
    }
}
