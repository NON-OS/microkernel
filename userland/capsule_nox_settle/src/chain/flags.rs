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

//! D11's flags of one address on its chain's NOX.

use nonos_nox_config::{address, Contract};
use nonos_settle_proto::{view, view_calldata, EINVAL, ENOENT, EPROTO};

use super::call::eth_calls;
use super::words::{addr_word, word_bool};
use crate::net::Rpc;

/// feeExempt(addr) and not isBlacklisted(addr), on the chain's NOX, in one
/// batch. Either call reverting is a no.
pub fn flags(rpc: &mut Rpc, chain_id: u64, addr: &[u8; 20]) -> Result<bool, i32> {
    let nox = address(chain_id, Contract::NoxToken).map_err(|_| ENOENT)?;
    let data = |id| view(id).and_then(|v| view_calldata(v, &[addr_word(addr)])).ok_or(EINVAL);
    match eth_calls(rpc, chain_id, &[(nox, data(3)?), (nox, data(4)?)]) {
        Ok(r) => Ok(matches!(r.as_slice(), [exempt, listed]
            if word_bool(exempt) == Some(true) && word_bool(listed) == Some(false))),
        Err(EPROTO) => Ok(false),
        Err(e) => Err(e),
    }
}
