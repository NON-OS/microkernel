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

//! The register requests on the wire, as r8152's get_registers and
//! set_registers send them (include/linux/usb/r8152.h: request 0x05,
//! 0xC0 to read, 0x40 to write, MCU_TYPE_PLA 0x0100, byte enables 0xff,
//! 0x33, 0x11 shifted with the word or byte).

use nonos_usbnet::mock::Call;
use nonos_usbnet::Setup;

use crate::chip::{rtl8153, RTL8153_IDS};
use crate::r8153::ocp::{read_byte, read_word, write_byte, write_dword, write_word, Dev};
use crate::r8153::ocp::{PLA, USB};

pub(crate) fn rd(addr: u16, index: u16) -> Call {
    Call::In(Setup::new(0xc0, 0x05, addr, index), 4)
}

pub(crate) fn wr(addr: u16, index: u16, data: [u8; 4]) -> Call {
    Call::Out(Setup::new(0x40, 0x05, addr, index), data.to_vec())
}

#[test]
fn each_access_is_one_aligned_dword_with_its_byte_enables() {
    let (bus, _) = rtl8153(0x5c10, RTL8153_IDS);
    let mut dev = Dev::new(bus.clone());
    assert_eq!(read_word(&mut dev, PLA, 0xe612), Ok(0x5c10));
    read_word(&mut dev, USB, 0xd406).unwrap();
    read_byte(&mut dev, PLA, 0xe813).unwrap();
    write_byte(&mut dev, PLA, 0xe813, 0x10).unwrap();
    write_byte(&mut dev, PLA, 0xe81c, 0xc0).unwrap();
    write_word(&mut dev, PLA, 0xe86c, 0xa000).unwrap();
    write_word(&mut dev, USB, 0xd406, 0x0010).unwrap();
    write_dword(&mut dev, PLA, 0xc010, 0x0e).unwrap();
    let want = [
        rd(0xe610, 0x01cc),
        rd(0xd404, 0x00cc),
        rd(0xe810, 0x0100),
        wr(0xe810, 0x0188, [0, 0, 0, 0x10]),
        wr(0xe81c, 0x0111, [0xc0, 0, 0, 0]),
        wr(0xe86c, 0x0133, [0x00, 0xa0, 0, 0]),
        wr(0xd404, 0x00cc, [0, 0, 0x10, 0]),
        wr(0xc010, 0x01ff, [0x0e, 0, 0, 0]),
    ];
    assert_eq!(bus.0.borrow().calls, want);
}
