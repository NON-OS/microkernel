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
        (Stage::NotClaimed, DriverStage::NotClaimed),
        (Stage::PowerFailed, DriverStage::PowerFailed),
        (Stage::FirmwareFailed, DriverStage::FirmwareFailed),
        (Stage::NoDma, DriverStage::NoDma),
        (Stage::EfuseFailed, DriverStage::EfuseFailed),
        (Stage::NoStationAddress, DriverStage::NoStationAddress),
        (Stage::NoAirPath, DriverStage::NoAirPath),
        (Stage::ChipInit, DriverStage::ChipInit),
    ];
    for (ours, theirs) in pairs {
        let read = DriverStage::from_code(ours as u8);
        assert!(read == theirs, "{ours:?} reads as {}", read.text());
    }
}

#[test]
fn the_client_asks_this_service_and_lets_it_join() {
    let me = services::SERVICES.iter().find(|s| s.name == b"driver.rtl8852be0");
    assert!(me.is_some_and(|s| s.joins && s.label == "Realtek RTL8852BE"));
    assert_eq!(services::CANNOT_JOIN, -38);
}
