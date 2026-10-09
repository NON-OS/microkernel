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

//! The serial console as the engine's log.

use nonos_libc::mk_debug;

use super::super::super::env::Log;
use super::super::super::text::LINE_MAX;

/// The serial console, whose tail the kernel keeps for `log` on a machine
/// without a serial port.
#[derive(Clone, Copy, Default)]
pub struct SerialLog;

impl Log for SerialLog {
    fn line(&self, text: &[u8]) {
        let mut buf = [0u8; LINE_MAX + 1];
        let n = core::cmp::min(text.len(), LINE_MAX);
        buf[..n].copy_from_slice(&text[..n]);
        buf[n] = b'\n';
        let _ = mk_debug(buf.as_ptr(), n + 1);
    }
}
