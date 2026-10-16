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

//! Before each intent, a contract's NOX flags are read again; one that now
//! fails is not deployed until the next start.

use nonos_nox_config::Contract;
use nonos_settle_proto::ENOENT;

use super::flags::flags;
use super::live::{Live, Mark};
use crate::net::Rpc;

impl Live {
    /// The address `usable` gives, its NOX flags read again now.
    pub fn fresh(&mut self, rpc: &mut Rpc, chain_id: u64, c: Contract) -> Result<[u8; 20], i32> {
        let at = self.usable(rpc, chain_id, c)?;
        if c == Contract::NoxToken || flags(rpc, chain_id, &at)? {
            return Ok(at);
        }
        *self.mark(chain_id, c)? = Mark::Bad;
        Err(ENOENT)
    }
}
