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

//! How much one OP_SEND can carry. A call's frame is 1536 bytes: the
//! 20-byte header, the 4-byte socket handle, then the payload. A mainnet
//! snapshot's TLS record is longer than that, so a direct socket sends
//! it in pieces of at most `SEND_MAX`, front first.

/// The call frame `call::call` builds.
pub const FRAME: usize = 1536;
/// The frame's header and the socket handle ahead of the payload.
pub const OVERHEAD: usize = 20 + 4;
pub const SEND_MAX: usize = FRAME - OVERHEAD;

/// The piece of `data` one send carries.
pub fn front(data: &[u8]) -> &[u8] {
    &data[..data.len().min(SEND_MAX)]
}
