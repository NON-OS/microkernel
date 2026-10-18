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
use crate::protocol::{RESP_HDR_LEN, STATUS_LEN};

/// The status word of a reply `len` bytes long in `tx`.
pub(super) fn status(tx: &[u8], len: usize) -> Option<i32> {
    if len < RESP_HDR_LEN + STATUS_LEN {
        return None;
    }
    let at = tx.get(RESP_HDR_LEN..RESP_HDR_LEN + STATUS_LEN)?;
    Some(i32::from_le_bytes([at[0], at[1], at[2], at[3]]))
}
