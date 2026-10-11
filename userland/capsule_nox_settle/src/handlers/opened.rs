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

//! OPENED: chain_hash(chain_id, txHash) -> the ChannelOpened event that
//! transaction emitted from the config Settlement. EAGAIN while it is not
//! mined (or its receipt is not this transaction's); ENOENT if it reverted or
//! emitted none.

use alloc::vec::Vec;

use nonos_eth_rpc::{parse_receipt_for, parse_receipt_logs, request_receipt};
use nonos_nox_config::Contract;
use nonos_settle_proto::{parse_chain_hash, Opened, EAGAIN, EINVAL, ENOENT, EPROTO};

use crate::settle::Settle;

pub fn opened(s: &mut Settle, body: &[u8]) -> Result<Vec<u8>, i32> {
    let ch = parse_chain_hash(body).ok_or(EINVAL)?;
    let settlement = s.live.usable(&mut s.rpc, ch.chain_id, Contract::Settlement)?;
    let resp = s.rpc.call(ch.chain_id, &request_receipt(&ch.id, 1))?;
    match parse_receipt_for(&resp, &ch.id) {
        None => return Err(EAGAIN),
        Some(false) => return Err(ENOENT),
        Some(true) => {}
    }
    let logs = parse_receipt_logs(&resp).ok_or(EPROTO)?;
    logs.iter()
        .filter(|l| l.address == settlement)
        .find_map(|l| Opened::from_log(&l.topics, &l.data))
        .map(|o| o.encode().to_vec())
        .ok_or(ENOENT)
}
