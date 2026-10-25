// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! What the worker is asked to do, what it answers, and what came of
//! handing it a request.

use super::super::state::WIFI_NET_MAX;
use super::secret::Secret;
use crate::wifi::{ConnectResult, ScanNetwork, ScanOutcome, ScanStats};
use alloc::boxed::Box;
use nonos_wifi_client::{Driver, Dropped};

/// What the worker asks the driver.
pub(super) enum Ask {
    Scan(Driver),
    /// Join `net`; keep it among the saved networks when `remember`.
    Join {
        driver: Driver,
        net: ScanNetwork,
        pass: Secret,
        remember: bool,
    },
}

/// The driver's answer.
pub(super) enum Answer {
    Scanned {
        nets: [ScanNetwork; WIFI_NET_MAX],
        count: usize,
        outcome: ScanOutcome,
        stats: ScanStats,
    },
    /// The join's outcome and, when remembering was asked for and the join
    /// went through, what remembering said and the network it dropped.
    Joined { result: ConnectResult, kept: Option<&'static str>, dropped: Option<Dropped> },
}

/// What a remember that went through says.
pub(crate) const REMEMBERED: &str = "Remembered, sealed with this machine's TPM key";

/// What became of a request handed to `send`.
pub(super) enum Sent {
    /// A worker has it; a tick will find the answer.
    Out,
    /// The last request is still with the driver; this one was dropped.
    Busy,
    /// No worker could be started (the kernel refused the thread, or its stack
    /// could not be mapped): the request comes back, for
    /// the window thread to run itself.
    Refused(Box<Ask>),
}
