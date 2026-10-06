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

use crate::browser::fetch::wire::Wire;

/// Say how a handshake flight ended and how much of it had arrived.
///
/// A handshake that stops carries no message of its own, and over the
/// mixnet a flight that completed, was refused or could not be read look
/// alike from the outside, so the reason and the byte count are said out.
pub fn flight<W: Wire>(w: &mut W, reason: &str, have: usize) {
    let line = alloc::format!("[BROWSER] tls flight {reason} bytes {have}\n");
    w.trace(line.as_bytes());
}
