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

//! Bytes into a family stream: to its peer's queue.

use super::table::Socks;

impl Socks {
    /// As many of `bytes` as the peer has room for, EAGAIN for none.
    pub fn write(&mut self, id: u32, bytes: &[u8]) -> Result<usize, i64> {
        self.put(id, bytes).map(|(n, _)| n)
    }
}
