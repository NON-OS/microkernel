/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

//! Where the Wi-Fi panel stands with a scan and with a join.

use crate::wifi::{ConnectResult, ScanOutcome};

/// Where the Wi-Fi panel stands with the driver: it has not scanned yet, the last
/// scan resolved to one of the driver outcomes.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum WifiScan {
    Idle,
    Done(ScanOutcome),
}

/// Where a connection attempt stands. `Failed` carries the driver's status code
/// so the panel can say why.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum WifiConnect {
    Idle,
    Connected,
    Failed(ConnectResult),
}
