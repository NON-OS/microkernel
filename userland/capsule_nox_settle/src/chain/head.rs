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
//! The head a log read pages to: the chain's block number from the current
//! host and from a second one, held to settle_proto's confirmed_head, so no
//! one host names how far settle pages.

use nonos_eth_rpc::{parse_u64, request_block_number};
use nonos_nox_config::Chain;
use nonos_settle_proto::{confirmed_head, ENOENT, EPROTO};

use crate::net::Rpc;

pub fn head(rpc: &mut Rpc, chain_id: u64) -> Result<u64, i32> {
    let chain = Chain::from_id(chain_id).ok_or(ENOENT)?;
    let ask = request_block_number(1);
    let first = parse_u64(&rpc.call(chain_id, &ask)?).ok_or(EPROTO)?;
    let second = parse_u64(&rpc.call_second(chain_id, &ask)?).ok_or(EPROTO)?;
    confirmed_head(chain, first, second).ok_or(EPROTO)
}
