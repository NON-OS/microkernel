/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! How far bring-up got: the stage byte the status op answers first, which
//! Settings and the setup wizard turn into a sentence through
//! `nonos_wifi_client::DriverStage` (the same numbers every Wi-Fi driver
//! uses). A driver that stops short still serves, so the panel shows where.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stage {
    /// The radio is up: the firmware runs and the station scans.
    Ready = 0,
    /// The card would not hand itself to the driver (driver own), or its
    /// subsystem reset never finished.
    PowerFailed = 2,
    /// The card read back all ones: it is not answering on the bus.
    DeadMmio = 3,
    /// The firmware files were right but the card did not take them, or
    /// did not answer once running (the log names the step).
    FirmwareFailed = 4,
    /// No DMA memory below 4 GiB for the rings.
    NoDma = 5,
    /// No randomness for the address the interface starts under.
    NoStationAddress = 7,
    /// A chip id this driver has no firmware for.
    NoAirPath = 8,
    /// The firmware package is not on the boot medium.
    PackageMissing = 10,
    /// A file of the package is there but is not the one this driver
    /// carries the hash of.
    PackageMismatch = 11,
}

impl Stage {
    pub fn text(self) -> &'static str {
        match self {
            Stage::Ready => "ready",
            Stage::PowerFailed => "power: the card did not hand over",
            Stage::DeadMmio => "the card's registers read back dead",
            Stage::FirmwareFailed => "firmware: the card did not take it",
            Stage::NoDma => "no DMA memory for the rings",
            Stage::NoStationAddress => "no randomness for an address",
            Stage::NoAirPath => "chip not supported",
            Stage::PackageMissing => "firmware package missing",
            Stage::PackageMismatch => "firmware package does not match",
        }
    }
}
