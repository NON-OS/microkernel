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
/// bring-up steps; 8 is what the iwlwifi driver reports for a card it takes but
/// has no air path for: everything but the SO-platform AX211 and AX201 (PCI
/// 0x51F0, 0x51F1, 0x54F0, 0x7A70, 0x7AF0, 0x7F70), whose firmware it boots.
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
            DriverStage::NoAirPath => "card not supported yet; use Ethernet or USB Wi-Fi",
            DriverStage::Unknown => "The driver reported an unknown stage",
        }
    }
}

/// Where the RTL8821CE's status reply puts the firmware failure step: after the
/// stage byte, the 52 bytes of counters and registers.
const FW_STEP_AT: usize = WIFI_HDR + 1 + 52;

impl Driver {
    /// The firmware failure step an RTL8821CE reports (see
    /// [`crate::firmware_step_text`]), or `None` when the driver did not answer
    /// or answers a shorter status (iwlwifi, or a driver built before it).
    pub fn firmware_step(&self) -> Option<u8> {
        let mut resp = [0u8; FW_STEP_AT + 1];
        match self.request(OP_STATUS, &[], &mut resp, STATUS_TIMEOUT_MS) {
            Some(n) if n > FW_STEP_AT => Some(resp[FW_STEP_AT]),
            _ => None,
        }
    }

    /// Ask how far bring-up got. `None` when the driver did not answer.
    pub fn stage(&self) -> Option<DriverStage> {
        self.stage_within(STATUS_TIMEOUT_MS)
    }

    /// `stage`, waiting at most `timeout_ms` for the answer: for a caller
    /// whose loop serves others (net_core) and must not wait out a busy driver.
    pub fn stage_within(&self, timeout_ms: u64) -> Option<DriverStage> {
        let mut resp = [0u8; WIFI_HDR + 1];
        match self.request(OP_STATUS, &[], &mut resp, timeout_ms) {
            Some(n) if n > WIFI_HDR => Some(DriverStage::from_code(resp[WIFI_HDR])),
            _ => None,
        }
    }
}
