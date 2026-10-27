// NONOS Operating System (AGPL-3.0-or-later)

//! Each stage the driver reports is the stage the panels read
//! (`nonos_wifi_client::DriverStage`, mounted here as the client builds it):
//! the same number means the same thing on both sides. And the client asks
//! this driver's service by its name, as one that joins.

#[path = "../../../nonos_wifi_client/src/driver/services.rs"]
mod services;
#[path = "../../../nonos_wifi_client/src/driver/stage_kind.rs"]
mod stage_kind;

use crate::stage::Stage;
use stage_kind::DriverStage;

#[test]
fn every_driver_stage_reads_as_its_own_on_the_panel() {
    let pairs = [
        (Stage::Ready, DriverStage::Ready),
        (Stage::PowerFailed, DriverStage::PowerFailed),
        (Stage::DeadMmio, DriverStage::DeadMmio),
        (Stage::FirmwareFailed, DriverStage::FirmwareFailed),
        (Stage::NoDma, DriverStage::NoDma),
        (Stage::NoStationAddress, DriverStage::NoStationAddress),
        (Stage::NoAirPath, DriverStage::NoAirPath),
        (Stage::PackageMissing, DriverStage::PackageMissing),
        (Stage::PackageMismatch, DriverStage::PackageMismatch),
    ];
    for (ours, theirs) in pairs {
        let read = DriverStage::from_code(ours as u8);
        assert!(read == theirs, "{ours:?} reads as {}", read.text());
    }
}

#[test]
fn the_client_asks_this_service_and_lets_it_join() {
    let me = services::SERVICES.iter().find(|s| s.name == b"driver.mt7921e0");
    assert!(me.is_some_and(|s| s.joins && s.label == "MediaTek MT7921/MT7922"));
    assert_eq!(services::CANNOT_JOIN, -38);
}

#[test]
fn a_medium_without_the_package_reads_missing_on_the_panel() {
    use crate::fw::package::{for_chip, FILES};
    use crate::model::rig::{chip, mapped, regs};
    FILES.iter().for_each(|f| nonos_libc::set_slot(f.kind, None));
    let c = chip();
    let (radio, stage) = crate::bringup::run_with(regs(&c), &mapped(), for_chip);
    assert!(radio.is_none() && stage == Stage::PackageMissing);
    let text = DriverStage::from_code(stage as u8).text();
    assert_eq!(text, "The firmware package is missing from the boot medium");
    assert!(c.borrow().mcu.seen.is_empty(), "nothing was sent to the card");
}
