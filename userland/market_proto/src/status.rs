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

//! Where the market is found, and the statuses it replies with, by name.

/// The service the market capsule announces itself under.
pub const SERVICE: &[u8] = b"market.index";

/// As `capsule_market`'s `protocol/errno.rs` defines them.
pub const E_INVAL: i32 = -22;
pub const E_NODATA: i32 = -61;
pub const E_MSGSIZE: i32 = -90;
pub const E_STALE: i32 = -116;
pub const E_KEYREJECTED: i32 = -129;

const NAMES: [(i32, &str); 5] = [
    (E_INVAL, "EINVAL"),
    (E_NODATA, "ENODATA"),
    (E_MSGSIZE, "EMSGSIZE"),
    (E_STALE, "ESTALE"),
    (E_KEYREJECTED, "EKEYREJECTED"),
];

/// "ENODATA" for -61; None for a status the market does not send.
pub fn status_name(status: i32) -> Option<&'static str> {
    NAMES.iter().find(|(n, _)| *n == status).map(|(_, name)| *name)
}
