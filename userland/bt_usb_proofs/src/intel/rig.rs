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

//! Controllers to load and a run of the loader against one. Each version
//! carries an OTP address, which no line said may hold.

use std::cell::RefCell;

use super::loader::{carried::carried, setup};
use super::model::Intel;
use crate::hci_sim::stack::IFACE;
use nonos_bt_hci::usb::UsbHci;
use nonos_bt_hci::{HciError, Host};

pub const OTP: [u8; 6] = [0xa1, 0xb2, 0xc3, 0xd4, 0xe5, 0xf6];
/// The CNVi/CNVr top registers, the variant and the carried file's name.
pub const TYP: (u32, u32, u8, &str) = (0x410, 0x410, 0x17, "ibt-0041-0041");
pub const SOLAR: (u32, u32, u8, &str) = (0x401, 0x410, 0x18, "ibt-1040-0041");
pub const BZRU: (u32, u32, u8, &str) = (0x0200_0910, 0x0200_0910, 0x1d, "ibt-0291-0291");

thread_local!(static SAID: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) });

fn say(words: &[&[u8]]) {
    let line = words.concat();
    assert!(!line.windows(6).any(|w| w == OTP), "an address said");
    SAID.with(|s| s.borrow_mut().push(String::from_utf8(line).expect("a line of text")));
}

/// Read Version's TLVs for a part, its image and build, engine `sbe`.
pub fn tlv(part: (u32, u32, u8, &str), img: u8, build: [u8; 3], sbe: u8) -> Vec<u8> {
    let bt = (u32::from(part.2) << 16 | 0x37 << 8).to_le_bytes();
    let mut v = [&[0x10, 4][..], &part.0.to_le_bytes(), &[0x11, 4], &part.1.to_le_bytes()].concat();
    v.extend([&[0x12, 4][..], &bt, &[0x1c, 1, img, 0x2d, 3], &build, &[0x2f, 1, sbe]].concat());
    v.extend([&[0x30, 6][..], &OTP].concat());
    v
}

/// A model of `part` in its bootloader, expecting its carried firmware.
pub fn bootloader(part: (u32, u32, u8, &str), sbe: u8) -> Intel {
    let sfi = carried(part.3).expect("carried").sfi;
    let mut m = Intel { boots: true, expect: sfi.len() - 964, ..Intel::default() };
    m.tlv = [tlv(part, 0x01, [0x10, 0x20, 0x1a], sbe), tlv(part, 0x03, [0x18, 0x1d, 0x1a], sbe)];
    m
}

/// The loader run against `m` as product `pid`: what it returned, the
/// model as it was left, and every line said.
pub fn run(m: Intel, pid: u16) -> (Result<(), HciError>, Intel, Vec<String>) {
    SAID.with(|s| s.borrow_mut().clear());
    let mut host = Host::new(UsbHci::new(m, &IFACE, 27));
    let r = setup(&mut host, pid, say);
    let m = std::mem::take(host.transport().pipes());
    (r, m, SAID.with(|s| s.borrow().clone()))
}

pub fn ops_after(m: &Intel, op: u16) -> Vec<u16> {
    let at = m.ops.iter().rposition(|&o| o == op).expect("sent");
    m.ops[at + 1..].to_vec()
}
