/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! How far the driver's radio bring-up got, from the status op.

use super::call::{OP_STATUS, WIFI_HDR};
use super::find::Driver;

/// The status reply comes back at once; no scan runs.
const STATUS_TIMEOUT_MS: u64 = 500;

/// The stage codes both drivers report. Codes 0 to 7 are the RTL8821CE's
/// bring-up steps; 8 is what the iwlwifi driver reports, because it brings
/// the card out of reset but never starts its firmware.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DriverStage {
    Ready,
    NotClaimed,
    PowerFailed,
    DeadMmio,
    FirmwareFailed,
    NoDma,
    EfuseFailed,
    NoStationAddress,
    NoAirPath,
    Unknown,
}

impl DriverStage {
    fn from_code(code: u8) -> Self {
        use DriverStage::*;
        const ALL: [DriverStage; 9] = [
            Ready,
            NotClaimed,
            PowerFailed,
            DeadMmio,
            FirmwareFailed,
            NoDma,
            EfuseFailed,
            NoStationAddress,
            NoAirPath,
        ];
        ALL.get(code as usize).copied().unwrap_or(Unknown)
    }

    /// One line a panel can show for this stage.
    pub fn text(self) -> &'static str {
        match self {
            DriverStage::Ready => "Ready",
            DriverStage::NotClaimed => "The card could not be claimed",
            DriverStage::PowerFailed => "The card did not power on",
            DriverStage::DeadMmio => "The card's registers read back dead",
            DriverStage::FirmwareFailed => "The card's firmware did not load",
            DriverStage::NoDma => "No DMA memory for the radio",
            DriverStage::EfuseFailed => "The card's calibration did not read",
            DriverStage::NoStationAddress => "No random address could be drawn",
            DriverStage::NoAirPath => "This driver cannot scan or join yet",
            DriverStage::Unknown => "The driver reported an unknown stage",
        }
    }
}

impl Driver {
    /// Ask how far bring-up got. `None` when the driver did not answer.
    pub fn stage(&self) -> Option<DriverStage> {
        let mut resp = [0u8; WIFI_HDR + 1];
        match self.request(OP_STATUS, &[], &mut resp, STATUS_TIMEOUT_MS) {
            Some(n) if n > WIFI_HDR => Some(DriverStage::from_code(resp[WIFI_HDR])),
            _ => None,
        }
    }
}
