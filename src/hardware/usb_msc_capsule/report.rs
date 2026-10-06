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


//! Where driver.usb_msc0's search for its device stands, read from the tail
//! of its state reply and said in words. The driver holds no Debug
//! capability; this is how a stick it did not take is named on the log.
//! The wire mirrors userland/capsule_driver_usb_msc/src/scan/report.rs.

use alloc::string::String;

use super::protocol::OP_GET_STATE;
use super::report_words::{words, REPORT_LEN};
use super::transport::round_trip;

/// The driver's counters come first; the report follows them.
const COUNTERS_LEN: usize = 48;

/// The search's last stage, as one "[USB-MSC] ..." line, or None when the
/// driver does not answer or predates the report.
pub fn report_line() -> Option<String> {
    let body = round_trip(OP_GET_STATE, &[]).ok()?;
    let r = body.get(COUNTERS_LEN..COUNTERS_LEN + REPORT_LEN)?;
    Some(words(r))
}

