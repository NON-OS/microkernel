// NONOS Operating System (AGPL-3.0-or-later)
//! Into and out of leisure power save through the firmware: Bluetooth told,
//! the power mode command in rtw88's layout, and the power management bit
//! the firmware owes the access point cleared if it never is.

use super::fw_card::{FwCard, TCR};
use crate::h2c::cmds::pwr_mode;
use crate::pcie::Aspm;
use crate::ps::{Ps, Seen, PERIOD_MS};

const SCBD: usize = 0x00AA;
const BOX0: usize = 0x01D0;
const BOX0_EX: usize = 0x01F0;
const QUIET: Seen = Seen { tx: 0, rx: 0, sends: 0 };

fn asleep(card: &FwCard) -> Ps {
    let mut ps = Ps::new(Aspm::default());
    ps.step(card, 0, QUIET, true);
    ps.step(card, PERIOD_MS, QUIET, true);
    ps
}

#[test]
fn the_power_mode_words_are_rtw88s() {
    assert_eq!(pwr_mode(true), (0x0121_0120, 0x0000), "LPS, RF off, smart PS 2");
    assert_eq!(pwr_mode(false), (0x0100_0020, 0x0C00), "active, all on");
}

#[test]
fn entering_tells_bluetooth_then_the_firmware() {
    let card = FwCard::new(false, Some(0));
    assert!(asleep(&card).asleep());
    let told = card.at(SCBD, 0x8002).expect("Bluetooth told");
    let (w0, w1) = pwr_mode(true);
    let (ex, cmd) = (card.at(BOX0_EX, w1).expect("ext"), card.at(BOX0, w0).expect("cmd"));
    assert!(told < ex && ex < cmd, "scoreboard, then the extended word, then the command");
}

#[test]
fn a_firmware_that_never_takes_the_command_leaves_the_station_awake() {
    let card = FwCard::new(true, Some(0));
    assert!(!asleep(&card).asleep());
    assert_eq!(card.wrote(SCBD), [0x8002, 0x8003], "and Bluetooth told it is awake");
}

#[test]
fn leaving_waits_for_the_firmware_and_tells_bluetooth_last() {
    let card = FwCard::new(false, Some(3));
    let mut ps = asleep(&card);
    ps.wake(&card);
    assert!(!ps.asleep());
    assert!(card.wrote(TCR).is_empty(), "the firmware cleared the bit itself");
    let (w0, _) = pwr_mode(false);
    assert!(card.at(BOX0 + 4, w0).is_some(), "the active command, in the next box");
    assert_eq!(card.writes.borrow().last(), Some(&(SCBD, 0x8003)));
}

#[test]
fn a_bit_the_firmware_never_clears_is_cleared_here() {
    let card = FwCard::new(false, None);
    let mut ps = asleep(&card);
    ps.wake(&card);
    assert_eq!(card.wrote(TCR), [0x00], "power management bit off");
}
