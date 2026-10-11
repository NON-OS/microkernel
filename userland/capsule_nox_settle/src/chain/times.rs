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

//! The escrow's and the bond's time constants against the config
//!: MAX_DELIVERY and DISPUTE_WINDOW exactly, and an UNBOND_DELAY
//! long enough for every funded order to end inside it. A contract that
//! answers otherwise, or reverts, is treated as not deployed.

use nonos_nox_config::{Contract, DISPUTE_WINDOW, MAX_DELIVERY, UNBOND_DELAY_MIN};
use nonos_settle_proto::EPROTO;

use super::call_one::view_call;
use super::words::word_u64;
use crate::net::Rpc;

fn read(rpc: &mut Rpc, chain_id: u64, at: &[u8; 20], view: u16) -> Result<Option<u64>, i32> {
    match view_call(rpc, chain_id, at, view, &[]) {
        Ok(w) => Ok(word_u64(&w)),
        Err(EPROTO) => Ok(None),
        Err(e) => Err(e),
    }
}

pub fn times_ok(rpc: &mut Rpc, chain_id: u64, c: Contract, at: &[u8; 20]) -> Result<bool, i32> {
    Ok(match c {
        Contract::Escrow => {
            read(rpc, chain_id, at, 23)? == Some(MAX_DELIVERY)
                && read(rpc, chain_id, at, 24)? == Some(DISPUTE_WINDOW)
        }
        Contract::PublisherBond => {
            read(rpc, chain_id, at, 34)?.is_some_and(|d| d >= UNBOND_DELAY_MIN)
        }
        _ => true,
    })
}
