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
//! up under. The reset reloads RAL0/RAH0 from the NVM, so the factory
//! address is put back before the check; entropy stays on and every draw
//! differs, so neither it nor a fresh draw passes, only the one kept from the
//! bring-up.

use nonos_libc::{entropy, logged};

use super::tx_hang_tests::stalled;
use crate::constants::regs::{REG_RAH0, REG_RAL0};
use crate::constants::rxtx::RAH_AV;
use crate::constants::status::STATUS_LU;
use crate::regs::Regs;
use crate::server::tx_watch::check;

fn rar0(regs: &Regs) -> (u32, u32) {
    // SAFETY: RAL0 and RAH0 lie inside the window.
    unsafe { (regs.r32(REG_RAL0), regs.r32(REG_RAH0)) }
}

#[test]
fn a_recovery_keeps_the_station_address_drawn_at_bring_up() {
    let _turn = entropy(true);
    let (_part, mut d, _mem) = stalled(STATUS_LU);
    let first = rar0(&d.regs);
    let factory = (0x5634_1200, RAH_AV | 0x9A78);
    assert_ne!(first, factory, "the bring-up drew its own address");
    // SAFETY: as above.
    unsafe {
        d.regs.w32(REG_RAL0, factory.0);
        d.regs.w32(REG_RAH0, factory.1);
    }
    assert!(check(&mut d, false), "a stall past the bound is a hang");
    assert_eq!(logged().last().unwrap(), "e1000e: tx hang, reset: done");
    assert_eq!(rar0(&d.regs), first, "RAL0/RAH0 hold the bring-up's address");
}
