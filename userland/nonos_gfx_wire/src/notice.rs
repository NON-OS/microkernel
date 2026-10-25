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

//! The one message a display driver sends unasked: a notice to the
//! compositor that owns its primaries, so it acts at once rather than on its
//! next 500 ms poll. It lands in the compositor's inbox among its clients'
//! requests, so it travels in the compositor's own envelope (NCMP v1: magic,
//! version, op OUTPUT_NOTICE 0x0012, flags, request id 0, payload length),
//! with one u32: why. Nothing answers it, so it can elicit nothing.

/// "NCMP", the compositor's envelope.
pub const NCMP_MAGIC: u32 = 0x4E43_4D50;
pub const NCMP_VERSION: u16 = 1;
pub const OP_OUTPUT_NOTICE: u16 = 0x0012;
pub const NOTICE_LEN: usize = 24;

/// An output went dark or came back: poll now.
pub const OUTPUTS_CHANGED: u32 = 1;
/// The driver's device left (DGON) and the driver is ending: drive another.
pub const LEAVING: u32 = 2;

pub fn notice(why: u32) -> [u8; NOTICE_LEN] {
    let mut b = [0u8; NOTICE_LEN];
    b[0..4].copy_from_slice(&NCMP_MAGIC.to_le_bytes());
    b[4..6].copy_from_slice(&NCMP_VERSION.to_le_bytes());
    b[6..8].copy_from_slice(&OP_OUTPUT_NOTICE.to_le_bytes());
    b[16..20].copy_from_slice(&4u32.to_le_bytes());
    b[20..24].copy_from_slice(&why.to_le_bytes());
    b
}

/// The reason a notice's body gives, None for anything else.
pub fn why(body: &[u8]) -> Option<u32> {
    let w = u32::from_le_bytes(body.try_into().ok()?);
    matches!(w, OUTPUTS_CHANGED | LEAVING).then_some(w)
}
