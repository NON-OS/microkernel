// NONOS Operating System (AGPL-3.0-or-later)
//! The stage byte names the step that stopped, and never claims more: a chip
//! that misbehaves at one step ends bring-up there, with that stage.

use crate::model::{rig, Card, State};
use crate::stage::Stage;

pub(super) fn stops(f: impl FnOnce(&mut State)) -> (Card, Stage, &'static [u8]) {
    match rig::bring(f) {
        (card, Err((_, stage, why))) => (card, stage, why),
        (_, Ok(_)) => panic!("came up on a chip that misbehaved"),
    }
}

pub(super) fn stage(f: impl FnOnce(&mut State)) -> Stage {
    stops(f).1
}

#[test]
fn a_chip_with_nothing_wrong_comes_up_clean() {
    let (card, _chip) = rig::ready();
    let s = card.0.borrow();
    assert_eq!(s.violations, Vec::<String>::new());
    assert!(s.powered, "left powered for the station");
    assert_eq!(s.fwdl, crate::model::cpu::Fwdl::Running);
}

#[test]
fn power_that_never_reads_ready_is_power_failed() {
    assert_eq!(stage(|s| s.faults.stuck.push((0x4, 0x2_0000, false))), Stage::PowerFailed);
}

#[test]
fn a_card_off_the_bus_is_power_failed() {
    assert_eq!(stage(|s| s.faults.gone = true), Stage::PowerFailed);
}

#[test]
fn a_dead_efuse_is_efuse_failed() {
    assert_eq!(stage(|s| s.faults.efuse_dead = true), Stage::EfuseFailed);
}

#[test]
fn a_firmware_checksum_failure_is_firmware_failed() {
    assert_eq!(stage(|s| s.faults.fw_status = Some(2)), Stage::FirmwareFailed);
}

#[test]
fn a_mute_mailbox_is_firmware_failed() {
    assert_eq!(stage(|s| s.faults.mute = true), Stage::FirmwareFailed);
}

#[test]
fn a_command_queue_that_never_moves_is_firmware_failed() {
    assert_eq!(stage(|s| s.faults.h2c_stall = true), Stage::FirmwareFailed);
}

#[test]
fn a_one_antenna_board_has_no_air_path() {
    let (_, st, why) = stops(|s| s.phy_cap[3] = 1 << 16);
    assert_eq!(
        (st, why),
        (Stage::NoAirPath, &b"[rtl8852be] one-antenna board not supported\n"[..])
    );
}

#[test]
fn mac_and_phy_steps_that_never_finish_are_chip_init() {
    assert_eq!(stage(|s| s.faults.stuck.push((0x8d00, 3, false))), Stage::ChipInit);
    assert_eq!(stage(|s| s.faults.stuck.push((0x9e10, 1 << 31, false))), Stage::ChipInit);
    assert_eq!(stage(|s| s.faults.stuck.push((0x18080, 0x4, false))), Stage::ChipInit);
}
