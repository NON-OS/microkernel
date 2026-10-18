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

//! The id list as text, and the memory windows a template names.

use super::descriptors::{mem32, qword, template};
use crate::legacy::Range;
use crate::platform_device::*;

#[test]
fn the_list_keeps_what_holds_and_counts_what_does_not() {
    let text = b"# id\tclass\tsource\n\nAMDI0010\ti2c\tunverified\r\n80860F14\tsd\tunverified: recalled\nINT33FC\tgpio\tx\nTOOLONGID\ti2c\tx\nINT-1\ti2c\tx\nINT1\tspi\tx\nINT2\ti2c\t\nINT3\ti2c\n";
    let (list, dropped) = known(text);
    let ids: Vec<(&[u8], Class)> = list.iter().map(|k| (k.id, k.class)).collect();
    assert_eq!(
        ids,
        [(&b"AMDI0010"[..], Class::I2c), (b"80860F14", Class::Sd), (b"INT33FC", Class::Gpio)]
    );
    assert_eq!(list[1].source, b"unverified: recalled", "the source is the rest of the line");
    assert_eq!(dropped, 5, "too long, not alphanumeric, unknown class, no source, no field");
}

#[test]
fn memory_windows_from_fixed_and_address_descriptors() {
    let r = |base, len| Range { base, len };
    let dword = [
        vec![0x87, 23, 0, 0, 0, 0],
        [0u32, 0xFE20_0000, 0xFE20_0FFF, 0, 0x1000].iter().flat_map(|v| v.to_le_bytes()).collect(),
    ]
    .concat();
    let io_space = [&dword[..3], &[1], &dword[4..]].concat();
    let t = template(&[
        mem32(0xFED0_0000, 0x400),
        qword(0x2_0000_0000, 0, 0x2000),
        dword.clone(),
        io_space,
        qword(0x3_0000_0000, 0x1000, 0x1000),
        mem32(0xFED1_0000, 0),
    ]);
    assert_eq!(
        windows(&t),
        [r(0xFED0_0000, 0x400), r(0x2_0000_0000, 0x2000), r(0xFE20_0000, 0x1000)],
        "an I/O-type window, a translated one and an empty one are left out"
    );
}
