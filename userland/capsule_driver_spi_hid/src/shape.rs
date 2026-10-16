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

//! An SPI input report laid out as the shared poll reads a HID-over-I2C
//! frame: a length le16 that counts itself, the report id when the device
//! numbers its reports, then the report. Pure, for the proofs.

/// The frame's length in `buf`, or `None` when it does not fit.
pub fn shape(id: u8, report: &[u8], buf: &mut [u8]) -> Option<usize> {
    let head = 2 + usize::from(id != 0);
    let n = head.checked_add(report.len())?;
    if n > buf.len() || n > usize::from(u16::MAX) {
        return None;
    }
    buf[..2].copy_from_slice(&(n as u16).to_le_bytes());
    if id != 0 {
        buf[2] = id;
    }
    buf[head..n].copy_from_slice(report);
    Some(n)
}
