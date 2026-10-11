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

//! What the driver reads before it writes: the id, the VRAM, the byte order.

use crate::constants::{
    DISPI_INDEX_ID, DISPI_INDEX_VIDEO_MEMORY_64K, QEXT_ENDIAN, QEXT_LITTLE_ENDIAN,
};
use crate::dispi::{id, little_endian, vram};
use crate::model::{Model, Op};

#[test]
fn only_a_bochs_dispi_id_is_an_adapter_to_drive() {
    for good in 0xB0C0..=0xB0CF_u16 {
        assert_eq!(id(&Model::with(&[(DISPI_INDEX_ID, good)])), Some(good));
    }
    for bad in [0, 0xFFFF, 0xB0B5, 0xB0D0, 0x0C0B] {
        assert_eq!(id(&Model::with(&[(DISPI_INDEX_ID, bad)])), None, "{bad:#x}");
    }
}

#[test]
fn vram_is_the_bar_capped_at_what_the_adapter_reports() {
    let bar = 16 << 20;
    assert_eq!(vram(&Model::default(), bar), bar, "no report: the BAR");
    let m = Model::with(&[(DISPI_INDEX_VIDEO_MEMORY_64K, 128)]);
    assert_eq!(vram(&m, bar), 8 << 20);
    assert_eq!(vram(&m, 4 << 20), 4 << 20, "never past the BAR");
}

#[test]
fn the_byte_order_is_set_only_where_the_register_is() {
    for (size, set) in
        [(0, false), (4, false), (7, false), (8, true), (0x200, true), (0x1001, false)]
    {
        let m = Model { qext_size: size, ..Model::default() };
        assert_eq!(little_endian(&m, 0x1000), set, "size {size}");
        let wrote = *m.log.borrow() == [Op::Qext(QEXT_ENDIAN, QEXT_LITTLE_ENDIAN)];
        assert_eq!(wrote, set, "size {size}");
    }
}
