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

//! Why a scan or a join the panel was asked for will not run. Each of these
//! returned without a word before, so Enter or C on the Wi-Fi page could do
//! nothing and the page looked as if it had not heard.

pub const RADIO_OFF: &str = "Wi-Fi is off; W turns it on";
pub const NO_DRIVER: &str = "No Wi-Fi driver is running";
pub const NOT_SCANNED: &str = "C joins a network from the scan list; Enter scans";

/// Why a scan will not run, if it will not.
pub fn scan_refusal(radio_on: bool) -> Option<&'static str> {
    (!radio_on).then_some(RADIO_OFF)
}

/// Why a join will not run, if it will not: the switch first, then the
/// driver, then whether the highlighted row is one the scan found.
pub fn join_refusal(radio_on: bool, driver: bool, on_scanned_row: bool) -> Option<&'static str> {
    if !radio_on {
        return Some(RADIO_OFF);
    }
    if !driver {
        return Some(NO_DRIVER);
    }
    (!on_scanned_row).then_some(NOT_SCANNED)
}
