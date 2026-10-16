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

//! A recovery puts the part back on the air under the address it was brought
//! up under. Another address is put in IDR before the check; entropy stays
//! on and every draw differs, so neither it nor a fresh draw passes, only the
//! one kept from the bring-up.

use nonos_libc::{entropy, logged};

use super::hang_tests::stalled;
use crate::constants::regs::REG_MAC0;
use crate::constants::MAC_LEN;
use crate::part::part_with;
use crate::pio::Pio;
use crate::server::tx_watch::check;

fn idr(pio: &Pio) -> [u8; MAC_LEN] {
    let mut mac = [0u8; MAC_LEN];
    for (i, b) in mac.iter_mut().enumerate() {
        *b = pio.r8(REG_MAC0 + i as u16).expect("IDR reads");
    }
    mac
}

#[test]
fn a_recovery_keeps_the_station_address_drawn_at_bring_up() {
    let _turn = entropy(true);
    let (_port, _resets) = part_with(false, None);
    let mut d = stalled();
    let first = idr(&d.pio);
    assert_eq!(first, d.mac, "the bring-up wrote its drawn address");
    d.pio.w32(REG_MAC0, 0x5634_1200).expect("IDR0-3");
    d.pio.w32(REG_MAC0 + 4, 0x9A78).expect("IDR4-5");
    assert!(check(&mut d, false), "a stall past the bound is a hang");
    assert_eq!(logged().last().unwrap(), "driver.rtl8139: tx hang, reset: done\n");
    assert_eq!(idr(&d.pio), first, "IDR holds the bring-up's address");
}
