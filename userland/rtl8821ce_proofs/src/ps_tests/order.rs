// NONOS Operating System (AGPL-3.0-or-later)
//! L1 follows the firmware: on only once it sleeps, off before it is woken.

use super::fw_card::FwCard;
use crate::h2c::cmds::pwr_mode;
use crate::pcie::Aspm;
use crate::ps::{Ps, Seen, PERIOD_MS};

const BOX0: usize = 0x01D0;
const QUIET: Seen = Seen { tx: 0, rx: 0, sends: 0 };

#[test]
fn l1_comes_after_the_firmware_sleeps_and_goes_before_it_wakes() {
    const DBI_GO: (usize, u32) = (0x03F2, 1);
    let card = FwCard::new(false, Some(0));
    let mut ps = Ps::new(Aspm { l1: true, clkreq: false });
    ps.step(&card, 0, QUIET, true);
    ps.step(&card, PERIOD_MS, QUIET, true);
    let sleep_cmd = card.at(BOX0, pwr_mode(true).0).expect("slept");
    let l1_on = card.at(DBI_GO.0, DBI_GO.1).expect("L1 on");
    assert!(sleep_cmd < l1_on, "L1 only once the firmware sleeps");
    card.writes.borrow_mut().clear();
    ps.wake(&card);
    let l1_off = card.at(DBI_GO.0, DBI_GO.1).expect("L1 off");
    assert!(l1_off < card.at(BOX0 + 4, pwr_mode(false).0).expect("woken"), "L1 first");
}
