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

//! The 32-bit accesses the driver makes to either page, decoded.

use super::device::Device;
use super::events::Event;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bar {
    Pt,
    Vd,
}

impl Device {
    pub fn read32(&mut self, bar: Bar, offset: usize) -> u32 {
        match (bar, offset) {
            // Revisions offered: 1 unless the test takes it away.
            (Bar::Vd, 0x00) => {
                if self.faults.no_rev1 {
                    0b10
                } else {
                    0b1
                }
            }
            (Bar::Vd, 0x08) => 1,
            (Bar::Vd, 0x20) => self.answer,
            _ => panic!("driver read {bar:?} offset {offset:#x}"),
        }
    }

    pub fn write32(&mut self, bar: Bar, offset: usize, v: u32) {
        match (bar, offset) {
            (Bar::Vd, 0x00) => self.vrrs = v,
            (Bar::Vd, 0x08) => self.uvrs = v,
            (Bar::Vd, 0x10) => self.dsa = (self.dsa & !0xFFFF_FFFF) | v as u64,
            (Bar::Vd, 0x18) => {
                self.dsa = (self.dsa & 0xFFFF_FFFF) | (v as u64) << 32;
                self.note(Event::Shared(self.dsa));
            }
            (Bar::Vd, 0x20) => self.command(v),
            (Bar::Vd, 0x28) => self.mac[..4].copy_from_slice(&v.to_le_bytes()),
            (Bar::Vd, 0x30) => {
                self.mac[4..].copy_from_slice(&v.to_le_bytes()[..2]);
                self.note(Event::Mac(self.mac));
            }
            (Bar::Pt, 0x000) => {
                self.masked = v & 1 != 0;
                self.note(Event::Masked);
            }
            (Bar::Pt, 0x600) => {
                self.note(Event::TxProd(v));
                self.transmit();
            }
            (Bar::Pt, 0x800) => self.note(Event::RxProd(v)),
            (Bar::Pt, 0xA00) => self.note(Event::RxProd2(v)),
            _ => panic!("driver wrote {v:#x} at {bar:?} offset {offset:#x}"),
        }
    }
}
