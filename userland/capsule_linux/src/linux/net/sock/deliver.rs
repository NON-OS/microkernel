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

//! Bytes from one socket into the family: a stream's to its peer, a
//! datagram to where `dest` says, binding the sender first as Linux does.

use super::gram_dest::Dest;
use super::table::Socks;
use super::types::Proto;

impl Socks {
    pub fn deliver(&mut self, id: u32, dest: Dest, bytes: &[u8]) -> Result<usize, i64> {
        match self.get(id).map(|s| s.proto) {
            Some(Proto::Stream) => self.write(id, bytes),
            _ => self.autobind(id).and_then(|()| self.send_gram(id, dest, bytes)),
        }
    }
}
