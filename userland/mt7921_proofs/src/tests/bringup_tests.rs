// NONOS Operating System (AGPL-3.0-or-later)

//! Bring-up end to end on the modelled chip: the stage reported when each
//! step goes wrong, the firmware tried three times, and the card left
//! quiet (its engine stopped, Bus Master off) wherever it stops.

use nonos_libc::{commands, set_dma_mode, set_random, take_lines, DmaMode};

use crate::model::images::{patch_7922, ram_7922};
use crate::model::package::{package, pick};
use crate::model::rig::{bring_up, chip, mapped, regs};
use crate::stage::Stage;

fn stage_with(f: impl FnOnce(&mut crate::model::Chip)) -> (Stage, crate::model::Shared) {
    let c = chip();
    f(&mut c.borrow_mut());
    let (radio, stage) = bring_up(&c);
    assert_eq!(radio.is_some(), stage == Stage::Ready);
    (stage, c)
}

#[test]
fn a_good_card_and_package_come_up_ready() {
    let (stage, c) = stage_with(|_| {});
    assert_eq!(stage, Stage::Ready);
    assert!(c.borrow().mcu.running && !c.borrow().mcu.sem_held);
}

#[test]
fn no_package_and_a_wrong_package_are_their_own_stages() {
    let c = chip();
    package(&patch_7922(), &ram_7922(), false);
    assert_eq!(crate::bringup::run_with(regs(&c), &mapped(), pick).1, Stage::PackageMissing);
    assert!(c.borrow().mcu.seen.is_empty() && c.borrow().reg(0xd4300 + 4) == 0, "no ring set");
    let mut other = ram_7922();
    package(&patch_7922(), &ram_7922(), true);
    other[10] ^= 0xff;
    nonos_libc::set_slot(0x25, Some(other));
    assert_eq!(crate::bringup::run_with(regs(&c), &mapped(), pick).1, Stage::PackageMismatch);
}

#[test]
fn power_identity_and_memory_failures_have_their_stages() {
    assert_eq!(stage_with(|c| c.faults.dead = true).0, Stage::DeadMmio);
    assert_eq!(stage_with(|c| c.faults.no_own = true).0, Stage::PowerFailed);
    assert_eq!(stage_with(|c| c.faults.no_reset = true).0, Stage::PowerFailed);
    assert_eq!(stage_with(|c| c.id.0 = 0x7663).0, Stage::NoAirPath);
    set_dma_mode(DmaMode::Refuse);
    let (no_dma, _) = stage_with(|_| {});
    set_dma_mode(DmaMode::Grant);
    assert_eq!(no_dma, Stage::NoDma);
}

#[test]
fn a_firmware_that_never_answers_is_tried_three_times_then_left_quiet() {
    let _ = take_lines();
    let (stage, c) = stage_with(|c| c.faults.silent = true);
    assert_eq!(stage, Stage::FirmwareFailed);
    assert_eq!(c.borrow().mcu.plain(0x10, 0).len(), 3, "three semaphore requests");
    assert_eq!(commands().last(), Some(&(1 << 1)), "memory space only: Bus Master off");
    assert_eq!(c.borrow().reg(0xd4208) & 0x5, 0, "the WFDMA stopped");
    let lines = take_lines();
    let want = "[mt7921] stopped at stage 4: firmware: the card did not take it\n";
    assert!(lines.iter().any(|l| l == want), "{lines:?}");
}

#[test]
fn no_randomness_leaves_the_radio_down_rather_than_on_a_fixed_address() {
    set_random(false);
    let (stage, c) = stage_with(|_| {});
    set_random(true);
    assert_eq!(stage, Stage::NoStationAddress);
    assert!(c.borrow().mcu.uni(0x01).is_empty(), "no interface was added");
}
