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

//! Bytes out of one family socket: a stream's next bytes, or its next
//! datagram with the name of whoever sent it.

use alloc::vec::Vec;

use crate::linux::abi::errno::EBADF;

use super::name::Peer;
use super::table::Socks;
use super::types::Proto;

/// The bytes, a datagram's length before it was cut, and its sender.
pub type Taken = (Vec<u8>, usize, Option<Peer>);

impl Socks {
    pub fn take(&mut self, id: u32, want: usize, peek: bool) -> Result<Taken, i64> {
        match self.get(id).map(|s| s.proto) {
            None => Err(EBADF),
            Some(Proto::Stream) => {
                let bytes = self.read(id, want, peek)?;
                let n = bytes.len();
                Ok((bytes, n, None))
            }
            Some(Proto::Dgram) => {
                let got = self.take_gram(id, want, peek)?;
                /* A socketpair's peer has no name, and Linux gives none. */
                let from = match got.from {
                    Peer::Unix(None) => None,
                    named => Some(named),
                };
                Ok((got.bytes, got.whole, from))
            }
        }
    }
}
