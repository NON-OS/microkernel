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

//! The ambient set (CoreExec, IPC, Memory) counts as held whatever a parent's
//! mask says: every process has it, so a child holding it gains nothing. One
//! bit beyond it is still missing. Booted, proof_io's mask was 0x118, with no
//! CoreExec bit, and the bound named CoreExec missing before this.

use crate::ambient::AMBIENT_CAPS;
use crate::kernel_core::process_spawn::capsule_spawn::from_vfs::load::bound::missing;

#[test]
fn the_ambient_set_is_held_by_every_parent() {
    assert_eq!(AMBIENT_CAPS, 0x1 | 0x8 | 0x10);
    for parent in [0, 0x8 | 0x10 | 0x100, 0x40] {
        assert_eq!(missing(AMBIENT_CAPS, parent), 0, "parent {parent:#x}");
        assert_eq!(missing(AMBIENT_CAPS | 0x4, parent), 0x4 & !parent);
    }
    assert_eq!(missing(0x59, 0x118), 0x40, "std_proof from proof_io: FileSystem, not CoreExec");
}
