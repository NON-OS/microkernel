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

//! The capsule's own lines, outside the engine.

use crate::ufs::env::Log;
use crate::ufs::text::Line;

use super::serial_log::SerialLog;

pub fn unlisted(vendor: u16, device: u16) {
    let mut l = Line::new();
    l.s(b"UFS host ").hex(vendor as u64).s(b":").hex(device as u64);
    l.s(b" is not in the host list, left alone");
    SerialLog.line(l.as_bytes());
}

/// A broker call refused with `r`.
pub fn refused(what: &[u8], r: i64) {
    let mut l = Line::new();
    l.s(what).s(b" refused (").dec(r.unsigned_abs()).s(b")");
    SerialLog.line(l.as_bytes());
}

pub fn host(vendor: u16, device: u16) {
    let mut l = Line::new();
    l.s(b"host ").hex(vendor as u64).s(b":").hex(device as u64);
    SerialLog.line(l.as_bytes());
}
