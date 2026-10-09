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

//! GET_REPORT for one feature report: the answer is a 2-byte length prefix,
//! then the report, id byte first.

use super::command::{Command, REPORT_MAX};
use crate::i2c_client::write_read;

/// The report read into `report`, id byte first; 0 when the device refused
/// or answered with a length that does not fit.
pub(super) fn get(
    port: u32,
    addr: u8,
    cmd: &Command,
    id: u8,
    report: &mut [u8; REPORT_MAX],
) -> usize {
    let head_len = cmd.head_len;
    let mut report_len = 0usize;
    let mut get = [0u8; 7];
    get[..head_len].copy_from_slice(&cmd.head[..head_len]);
    get[head_len..head_len + 2].copy_from_slice(&cmd.data);
    let mut rx = [0u8; 64];
    if let Some(n) = write_read(port, addr, &get[..head_len + 2], &mut rx) {
        if n >= 3 {
            let total = usize::from(u16::from_le_bytes([rx[0], rx[1]]));
            if (3..=n.min(REPORT_MAX + 2)).contains(&total) {
                report_len = total - 2;
                report[..report_len].copy_from_slice(&rx[2..total]);
            }
        }
    }
    if report_len != 0 && report[0] != id && report_len < REPORT_MAX {
        // Device answered without the leading report-id byte; reinstate it so
        // the offsets and the SET_REPORT payload line up.
        report.copy_within(0..report_len, 1);
        report[0] = id;
        report_len += 1;
    }
    report_len
}
