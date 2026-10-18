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
//! Which router frame is its notice that a Bluetooth device paired without
//! authentication was refused on the secure path (G-76), rather than an
//! event: the envelope's flags say so (capsule_input_router
//! protocol/delivery.rs NOTICE_UNPROVEN). Held to the router's frames by
//! input_proofs.

pub const NOTICE_UNPROVEN: u16 = 1;

/// Whether `frame`, a delivery from the router, is that notice.
pub fn is_notice(frame: &[u8]) -> bool {
    frame.get(6..8).is_some_and(|f| u16::from_le_bytes([f[0], f[1]]) == NOTICE_UNPROVEN)
}
