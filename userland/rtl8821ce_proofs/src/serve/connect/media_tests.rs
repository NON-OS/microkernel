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

//! Linking the MAC to a BSS before the handshake names its BSSID and sets
//! the linked network type, the engine enables kept; unlinking forgets both.

use std::cell::RefCell;
use std::collections::HashMap;

use super::media::{link_to_bss, set_media_no_link};
use crate::constants::regs::{NETTYPE_MASK, NETTYPE_SHIFT, NET_TYPE_LINKED, NET_TYPE_NO_LINK};
use crate::constants::regs::{REG_BSSID, REG_CR};
use crate::regs::Mmio;

/// Registers by offset, each keeping what was last written there.
#[derive(Default)]
struct Regs(RefCell<HashMap<usize, u32>>);

impl Mmio for Regs {
    fn read8(&self, off: usize) -> u8 {
        self.read32(off) as u8
    }
    fn write8(&self, off: usize, val: u8) {
        self.write32(off, val.into())
    }
    fn read16(&self, off: usize) -> u16 {
        self.read32(off) as u16
    }
    fn write16(&self, off: usize, val: u16) {
        self.write32(off, val.into())
    }
    fn read32(&self, off: usize) -> u32 {
        self.0.borrow().get(&off).copied().unwrap_or(0)
    }
    fn write32(&self, off: usize, val: u32) {
        self.0.borrow_mut().insert(off, val);
    }
}

#[test]
fn linking_to_a_bss_names_it_and_unlinking_forgets_it() {
    let regs = Regs::default();
    regs.write32(REG_CR, 0xFF);
    let mut beacon = [0u8; 24];
    beacon[16..22].copy_from_slice(&[1, 2, 3, 4, 5, 6]);
    link_to_bss(&regs, &beacon);
    let net = |r: &Regs| (r.read32(REG_CR) & NETTYPE_MASK) >> NETTYPE_SHIFT;
    assert_eq!((regs.read32(REG_BSSID), regs.read16(REG_BSSID + 4)), (0x0403_0201, 0x0605));
    assert_eq!((net(&regs), regs.read32(REG_CR) & 0xFF), (NET_TYPE_LINKED, 0xFF), "engines kept");
    set_media_no_link(&regs);
    assert_eq!(
        (regs.read32(REG_BSSID), regs.read16(REG_BSSID + 4), net(&regs)),
        (0, 0, NET_TYPE_NO_LINK)
    );
}
