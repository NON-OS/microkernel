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

//! READ: one view of settle's table on a config contract, as words.

use alloc::vec::Vec;

use nonos_payment_proto::EIO;
use nonos_settle_proto::{parse_read_reply, Read, OP_READ};

use super::call::settle_call;
use super::chain::ROUTED_MS;

/// The view's raw return, at most `words` words.
pub(crate) fn read(
    chain_id: u64,
    view: u16,
    args: &[[u8; 32]],
    words: usize,
) -> Result<Vec<[u8; 32]>, i32> {
    let body = Read { chain_id, view, args: args.to_vec() }.encode().ok_or(EIO)?;
    let b = settle_call(OP_READ, &body, 1 + 32 * words, ROUTED_MS)?;
    parse_read_reply(&b).ok_or(EIO)
}

/// A uint word as u64, refusing one that does not fit.
pub(crate) fn word_u64(w: &[u8; 32]) -> Option<u64> {
    w[..24].iter().all(|b| *b == 0).then(|| u64::from_be_bytes(w[24..].try_into().unwrap()))
}
