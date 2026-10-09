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

//! PHY access against a modelled PHY: the address and page each family and
//! page take, Linux's retry on pch_mtp and later only, and the MDIC timeout.

use std::time::Instant;

use crate::constants::phy::{CV_SMB_CTRL, MII_BMCR};
use crate::constants::Family;
use crate::model::live::live;
use crate::model::part::Behaviour;
use crate::model::window::{i82574, pch, phy_at};
use crate::phy::access::{read, write};
use crate::regs::Regs;

#[test]
fn pch_page_zero_is_address_2_and_high_pages_are_selected_at_address_1() {
    let bar = pch();
    let part = live(&bar, phy_at(2), Behaviour::default());
    part.phy.set(1, 769, 23, 0x0001);
    let regs = Regs::new(bar.base());
    assert_eq!(read(&regs, Family::PchCnp, MII_BMCR), Ok(0x1140));
    assert_eq!(read(&regs, Family::PchCnp, CV_SMB_CTRL), Ok(0x0001));
    write(&regs, Family::PchCnp, CV_SMB_CTRL, 0).expect("written");
    let seen = part.phy.seen.lock().unwrap().clone();
    // BMCR read at PHY 2; page 769 selected at PHY 1, register 23 read and
    // written there.
    assert_eq!(seen, [0x0840_0000, 0x043F_6020, 0x0837_0000, 0x043F_6020, 0x0437_0000]);
    assert_eq!(part.phy.get(1, 769, 23), 0);
}

#[test]
fn the_82574_phy_is_address_1_unpaged() {
    let bar = i82574();
    let part = live(&bar, phy_at(1), Behaviour::default());
    assert_eq!(read(&Regs::new(bar.base()), Family::I82574, MII_BMCR), Ok(0x1140));
    assert_eq!(part.phy.seen.lock().unwrap().clone(), [0x0820_0000]);
}

#[test]
fn only_pch_mtp_and_later_retry_a_failed_transaction() {
    let bar = pch();
    let part = live(&bar, phy_at(2), Behaviour::default());
    let regs = Regs::new(bar.base());
    part.phy.fail_next.store(2, std::sync::atomic::Ordering::SeqCst);
    let start = Instant::now();
    assert_eq!(read(&regs, Family::PchMtp, MII_BMCR), Ok(0x1140), "third try answers");
    assert!(start.elapsed().as_millis() >= 20, "two 10 ms pauses between tries");
    part.phy.fail_next.store(1, std::sync::atomic::Ordering::SeqCst);
    let once = read(&regs, Family::PchTgp, MII_BMCR);
    assert_eq!(once, Err("MDIC error: the PHY did not answer"), "pch_tgp tries once");
}

#[test]
fn a_part_that_never_sets_ready_times_out_on_the_clock() {
    let bar = pch();
    let start = Instant::now();
    let r = read(&Regs::new(bar.base()), Family::PchLpt, MII_BMCR);
    assert_eq!(r, Err("MDIC not ready in 100 ms"));
    assert!(start.elapsed().as_millis() >= 100);
}
