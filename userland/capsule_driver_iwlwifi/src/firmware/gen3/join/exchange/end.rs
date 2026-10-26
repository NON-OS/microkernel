// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! How an exchange ended without a link.

use nonos_wifi_core::mlme::{Mlme, MlmeFailure};

use super::super::fw::CommandFailed;

/// How an exchange ended without a link.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ExchangeEnd {
    /// The MLME failed the join: the access point refused, or the network
    /// cannot be joined as it is. Or an SAE exchange ran out of time, with
    /// why its last frame was discarded.
    Refused(MlmeFailure),
    /// The access point stopped answering.
    TimedOut,
    /// A firmware command failed, or the firmware raised its error cause.
    Firmware(Option<CommandFailed>),
}

/// The exchange's time, or a step's own (`Mlme::deadline`), is spent: an
/// SAE exchange says why it never finished (a confirm that did not verify
/// reads as a wrong password), any other stopped answering.
pub fn timed_out(mlme: &Mlme) -> ExchangeEnd {
    match mlme.failure() {
        None | Some(MlmeFailure::Timeout(_)) => ExchangeEnd::TimedOut,
        Some(f) => ExchangeEnd::Refused(f),
    }
}
