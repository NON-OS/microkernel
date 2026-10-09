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

//! Ask vfs_pool whether the on-disk capsule store decoded at boot. None means
//! no answer yet; otherwise (code, settled): code 0 healthy, any other code the
//! boot failure class, and settled false while staging is still in flight, when
//! a non-zero code is only transient (the block device may not be ready yet).

use alloc::vec;

use super::call::call;
use super::constants::{HDR_LEN, OP_STORE_STATUS};

pub fn store_status() -> Option<(u32, bool)> {
    let mut rx = vec![0u8; HDR_LEN + 12];
    let total = call(OP_STORE_STATUS, &[], &mut rx)?;
    if total < HDR_LEN + 12 {
        return None;
    }
    let off = HDR_LEN + 4;
    let code = u32::from_le_bytes([rx[off], rx[off + 1], rx[off + 2], rx[off + 3]]);
    let settled = (rx[off + 4] | rx[off + 5] | rx[off + 6] | rx[off + 7]) != 0;
    Some((code, settled))
}
