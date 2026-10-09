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

//! How many bytes one read may hand a caller.

use crate::server::handlers::io::u32_at;

/// The most one read hands back. The service's buffers are sized for it.
pub const RECV_MAX: usize = 32 * 1024;

/// What a caller that does not state a capacity was built against.
const ASSUMED_CAP: usize = 1536;

/// The capacity the caller states after the handle and read number,
/// bounded by RECV_MAX. A read takes bytes out of the socket, so handing a
/// caller more than it can hold loses the rest; a caller too old to state
/// one is held to what it was built against.
pub fn recv_cap(body: &[u8]) -> usize {
    match u32_at(body, 8) {
        Ok(n) => (n as usize).clamp(1, RECV_MAX),
        Err(_) => ASSUMED_CAP,
    }
}
