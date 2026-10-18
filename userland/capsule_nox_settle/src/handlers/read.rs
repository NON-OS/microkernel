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

//! READ: one view of the table, on a config contract that passed its
//! checks; the raw ABI return as words.

use alloc::vec::Vec;

use nonos_settle_proto::{read_reply, view, view_calldata, Read, EFBIG, EINVAL, EPROTO};

use crate::chain::eth_call;
use crate::settle::Settle;

pub fn read(s: &mut Settle, body: &[u8]) -> Result<Vec<u8>, i32> {
    let r = Read::parse(body).ok_or(EINVAL)?;
    let v = view(r.view).ok_or(EINVAL)?;
    let data = view_calldata(v, &r.args).ok_or(EINVAL)?;
    let at = s.live.usable(&mut s.rpc, r.chain_id, v.contract)?;
    let ret = eth_call(&mut s.rpc, r.chain_id, &at, data)?;
    let (words, []) = ret.as_chunks::<32>() else { return Err(EPROTO) };
    read_reply(words).ok_or(EFBIG)
}
