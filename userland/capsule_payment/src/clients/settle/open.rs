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

//! Reading what the meter opened (OPEN CONFIRM).

use nonos_payment_proto::EIO;
use nonos_settle_proto::{chain_hash, Opened, OPENED_LEN, OP_OPENED};

use super::call::settle_call;
use super::chain::ROUTED_MS;

/// The ChannelOpened `tx` emitted: EAGAIN while unmined, ENOENT if none.
pub(crate) fn opened(chain_id: u64, tx: &[u8; 32]) -> Result<Opened, i32> {
    let b = settle_call(OP_OPENED, &chain_hash(chain_id, tx), OPENED_LEN, ROUTED_MS)?;
    Opened::parse(&b).ok_or(EIO)
}
