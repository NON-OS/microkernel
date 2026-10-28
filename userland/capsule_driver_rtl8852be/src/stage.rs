// NONOS Operating System (AGPL-3.0-or-later)

//! The stage byte the status op reports: `nonos_wifi_client`'s
//! `DriverStage` codes, each naming the step that stopped bring-up, and
//! code 12 for a PCIe, MAC, PHY or calibration step the chip did not
//! finish (no older code names that step; reporting one would
//! send whoever reads it to the wrong part of the chip).

use crate::bring::Stop;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stage {
    Ready = 0,
    NotClaimed = 1,
    PowerFailed = 2,
    FirmwareFailed = 4,
    NoDma = 5,
    EfuseFailed = 6,
    NoStationAddress = 7,
    /// A board this driver takes but cannot run: a single-antenna variant.
    NoAirPath = 8,
    ChipInit = 12,
}

pub fn stage_of(stop: Stop) -> Stage {
    match stop {
        Stop::Power(_) => Stage::PowerFailed,
        Stop::Firmware(_) | Stop::Mailbox(_) => Stage::FirmwareFailed,
        Stop::Efuse(_) => Stage::EfuseFailed,
        Stop::Pcie(_) | Stop::Mac(_) | Stop::Send(_) | Stop::Table(_) => Stage::ChipInit,
        Stop::Xtal | Stop::Coex | Stop::Nctl(_) | Stop::Sch(_) | Stop::NoChannel => Stage::ChipInit,
    }
}
