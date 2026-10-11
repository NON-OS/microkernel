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

//! Sending what the meter signed, and reading what it opened.

use alloc::vec::Vec;

use nonos_payment_proto::EIO;
use nonos_settle_proto::{chain_hash, Opened, OPENED_LEN, OP_BROADCAST, OP_OPENED};

use super::call::settle_call;
use super::chain::ROUTED_MS;

/// The hash of `raw`, once settle has sent it.
pub(crate) fn broadcast(chain_id: u64, raw: &[u8]) -> Result<[u8; 32], i32> {
    let mut body = Vec::with_capacity(8 + raw.len());
    body.extend_from_slice(&chain_id.to_le_bytes());
    body.extend_from_slice(raw);
    let b = settle_call(OP_BROADCAST, &body, 32, ROUTED_MS)?;
    b.as_slice().try_into().map_err(|_| EIO)
}

/// The ChannelOpened `tx` emitted: EAGAIN while unmined, ENOENT if none.
pub(crate) fn opened(chain_id: u64, tx: &[u8; 32]) -> Result<Opened, i32> {
    let b = settle_call(OP_OPENED, &chain_hash(chain_id, tx), OPENED_LEN, ROUTED_MS)?;
    Opened::parse(&b).ok_or(EIO)
}
