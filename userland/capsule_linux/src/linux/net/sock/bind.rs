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

//! A local address for a socket that sends or connects before it binds.

use crate::linux::abi::errno::EADDRNOTAVAIL;

use super::table::Socks;
use super::types::{Addr, Domain};

/// The loopback route's source address.
const LOOPBACK: [u8; 4] = [127, 0, 0, 1];

impl Socks {
    /// Bind `id` to 127.0.0.1 and a free ephemeral port, as Linux's
    /// autobind does, unless it is bound already or is a socketpair end,
    /// which has no address.
    pub fn autobind(&mut self, id: u32) -> Result<(), i64> {
        let unbound = |s: &&super::types::Sock| s.local.is_none() && s.domain == Domain::Inet;
        let Some(proto) = self.get(id).filter(unbound).map(|s| s.proto) else {
            return Ok(());
        };
        let port = self.ephemeral(proto, LOOPBACK).ok_or(EADDRNOTAVAIL)?;
        if let Some(s) = self.get_mut(id) {
            s.local = Some(Addr { ip: LOOPBACK, port });
        }
        Ok(())
    }
}
