/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The stage codes the drivers report, and a line a panel can show for each.

/// The stage codes both drivers report. Codes 0 to 7 are the RTL8821CE's
/// bring-up steps; 8 is what the iwlwifi driver reports for a card it takes but
/// has no air path for: everything but the SO-platform AX211 and AX201 (PCI
/// 0x51F0, 0x51F1, 0x54F0, 0x7A70, 0x7AF0, 0x7F70), whose firmware it boots;
/// 9 is iwlwifi restarting a firmware that raised its error cause.
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
    Restarting,
    Unknown,
}

impl DriverStage {
    pub(super) fn from_code(code: u8) -> Self {
        use DriverStage::*;
        const ALL: [DriverStage; 10] = [
            Ready,
            NotClaimed,
            PowerFailed,
            DeadMmio,
            FirmwareFailed,
            NoDma,
            EfuseFailed,
            NoStationAddress,
            NoAirPath,
            Restarting,
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
            DriverStage::Restarting => "The card's firmware stopped; restarting it",
            DriverStage::Unknown => "The driver reported an unknown stage",
        }
    }
}
