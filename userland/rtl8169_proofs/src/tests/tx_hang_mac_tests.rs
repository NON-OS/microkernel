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
//! up under. The factory address is put back in IDR before the check;
//! entropy stays on and every draw differs, so neither it nor a fresh draw
//! passes, only the one kept from the bring-up.

use std::sync::{atomic::AtomicU32, Arc};

use nonos_libc::{entropy, logged};

use super::model::{idr, FACTORY};
use super::tx_hang_tests::stalled;
use crate::constants::regs::REG_MAC0;
use crate::server::tx_watch::check;

#[test]
fn a_recovery_keeps_the_station_address_drawn_at_bring_up() {
    let (_turn, resets) = (entropy(true), Arc::new(AtomicU32::new(0)));
    let (bar, _part, mut d, _mem) = stalled(&resets);
    let first = idr(&bar);
    assert_ne!(first, FACTORY, "the bring-up drew its own address");
    for (i, b) in FACTORY.iter().enumerate() {
        bar.present8(REG_MAC0 + i, *b);
    }
    assert!(check(&mut d, true), "a stall past the bound is a hang");
    assert_eq!(logged().last().unwrap(), "rtl8169: tx hang, reset: done\n");
    assert_eq!(idr(&bar), first, "IDR holds the bring-up's address");
}
