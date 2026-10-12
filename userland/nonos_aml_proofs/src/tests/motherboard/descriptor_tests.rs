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

//! Each descriptor a motherboard device may use, read to the range firmware
//! keeps; what names no address, or names one through a translation, adds
//! nothing; and an empty template reserves nothing.

use super::build::*;
use crate::motherboard::{reserved, Reserved, Space};

fn io(base: u64, len: u64) -> Reserved {
    Reserved { space: Space::Io, base, len }
}

fn mem(base: u64, len: u64) -> Reserved {
    Reserved { space: Space::Memory, base, len }
}

#[test]
fn every_descriptor_kind() {
    let t = template(&[
        super::build::io(0x2E, 0x2E, 2),
        fixed_io(0x480, 0x10),
        mem32_fixed(0xE000_0000, 0x1000_0000),
        mem32(0xFED1_0000, 0xFED1_0000, 0x8000),
        mem24(0x0E00, 0x0E00, 0x0002),
        qword(0, 0x40_0000_0000, 0, 0x10_0000),
        dword(1, 0x1800, 0x100),
        word(1, 0x0CF8, 8),
        extended(0, 0xFE00_0000, 0x1_0000),
    ]);
    let want = [
        io(0x2E, 2),
        io(0x80, 0x10),
        mem(0xE000_0000, 0x1000_0000),
        mem(0xFED1_0000, 0x8000),
        mem(0x0E_0000, 0x200),
        mem(0x40_0000_0000, 0x10_0000),
        io(0x1800, 0x100),
        io(0x0CF8, 8),
        mem(0xFE00_0000, 0x1_0000),
    ];
    assert_eq!(reserved(&t), want, "fixed I/O decodes ten bits: 0x480 is port 0x80");
}

#[test]
fn a_relocatable_range_keeps_its_whole_span() {
    let t = template(&[super::build::io(0x400, 0x4F0, 0x10), mem32(0x1000, 0x3000, 0x1000)]);
    assert_eq!(reserved(&t), [io(0x400, 0x100), mem(0x1000, 0x3000)]);
}

#[test]
fn what_is_no_address_adds_nothing() {
    let t = template(&[
        qword(2, 0, 0, 0x100),
        qword(0, 0x8000_0000, 0x1000, 0x1000),
        qword(0, u64::MAX - 0xFFF, 0, 0x2000),
        mem32_fixed(0xFED0_0000, 0),
        dword(1, 0xFFF0, 0x20),
        vec![0x8A, 20, 0, 0],
    ]);
    assert_eq!(reserved(&t), [], "bus range, translated, wrapping, empty, past 0xFFFF, short");
    assert_eq!(reserved(&template(&[])), []);
    assert_eq!(reserved(&[]), []);
}
