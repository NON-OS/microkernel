// NONOS Operating System (AGPL-3.0-or-later)
//! The last two stages: a kernel with no entropy leaves the station with
//! no address to send from, and a chip that stopped is powered off by
//! `give_back` before the station serves its stage.

use super::stage_tests::{stage, stops};
use crate::setup::give_back;
use crate::stage::Stage;

#[test]
fn no_entropy_is_no_station_address() {
    nonos_libc::run_dry(true);
    let st = stage(|_| {});
    nonos_libc::run_dry(false);
    assert_eq!(st, Stage::NoStationAddress);
}

#[test]
fn a_mac_init_that_fails_powers_the_chip_off_itself() {
    // The DMAC's SS init never finishes: mac_init's trx_init fails.
    let (card, st, _) = stops(|s| s.faults.stuck.push((0x9e10, 1 << 31, false)));
    assert_eq!(st, Stage::ChipInit);
    assert!(!card.0.borrow().powered, "rtw89_mac_init's error path");
}

#[test]
fn giving_back_powers_the_chip_off() {
    let (card, st, why) = stops(|s| s.faults.efuse_dead = true);
    assert!(card.0.borrow().powered);
    let _: (Option<()>, Stage) = give_back(&card, 1, 1, st, why);
    assert!(!card.0.borrow().powered, "never left powered while serving down");
}
