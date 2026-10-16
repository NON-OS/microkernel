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
//! up under. The reset reloads the factory address into RAR0; entropy stays
//! on and every draw differs, so neither it nor a fresh draw passes, only the
//! one kept from the bring-up.

use nonos_libc::{entropy, said};

use super::probe::FACTORY;
use super::tx_hang_tests::stalled;
use crate::constants::ctrl::STATUS_LU;
use crate::constants::regs::{REG_RAH0, REG_RAL0};
use crate::regs::Regs;
use crate::server::tx_watch::check;

fn rar0(regs: &Regs) -> (u32, u32) {
    // SAFETY: the window covers the receive-address registers.
    unsafe { (regs.r32(REG_RAL0), regs.r32(REG_RAH0)) }
}

#[test]
fn a_recovery_keeps_the_station_address_drawn_at_bring_up() {
    let _turn = entropy(true);
    let (_part, _resets, mut d, _mem) = stalled(STATUS_LU);
    let first = rar0(&d.regs);
    assert_ne!(first, FACTORY, "the bring-up replaced the factory address");
    assert!(check(&mut d, false), "a stall past the bound is a hang");
    assert_eq!(said().last().unwrap(), "igc: tx hang, reset: done\n");
    assert_eq!(rar0(&d.regs), first, "RAR0 holds the bring-up's address");
}
