// NONOS Operating System (AGPL-3.0-or-later)

//! What the worker asks the driver, and what it answers.

use nonos_wifi_client::Driver;

use super::super::state::WIFI_NET_MAX;
use super::secret::Secret;
use crate::wifi::{ConnectResult, ScanNetwork, ScanOutcome, ScanStats};

/// What the worker asks the driver.
pub(in super::super) enum Ask {
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
pub(in super::super) enum Answer {
    Scanned {
        nets: [ScanNetwork; WIFI_NET_MAX],
        count: usize,
        outcome: ScanOutcome,
        stats: ScanStats,
    },
    /// The join's outcome and, when remembering was asked for and the join
    /// went through, what remembering said.
    Joined { result: ConnectResult, kept: Option<&'static str> },
}
