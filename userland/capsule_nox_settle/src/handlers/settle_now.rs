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

//! SETTLE (op 10): chain_id u64 -> one drain round now (Settled): the
//! settle() broadcast and what it carries, or ENOENT when nothing is owed.
//! The installer calls it after a PAY for an install, and goes on only when
//! the round carries that voucher (v1.4.7).

use alloc::vec::Vec;

use nonos_settle_proto::{EFBIG, EINVAL};

use crate::drain::round;
use crate::settle::Settle;

pub fn settle_now(s: &mut Settle, body: &[u8]) -> Result<Vec<u8>, i32> {
    let chain_id = u64::from_le_bytes(body.try_into().map_err(|_| EINVAL)?);
    round(s, chain_id)?.encode().ok_or(EFBIG)
}
