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

//! The broker's record types as `legacy_ide` names them.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Bar {
    pub base: u64,
    pub size: u64,
    pub kind: u8,
    pub flags: u8,
    pub aux: u32,
    pub _pad: [u8; 2],
}

#[repr(u8)]
pub enum BarKind {
    Pio = 2,
}

#[path = "../../../../../src/hardware/broker/table/legacy_ide.rs"]
mod legacy_ide;

/// `legacy_ide::fill`, which is the table's alone in the kernel.
pub fn fill(class: u8, subclass: u8, progif: u8, bars: &mut [Bar; 6], span: &mut u8) {
    legacy_ide::fill(class, subclass, progif, bars, span);
}
