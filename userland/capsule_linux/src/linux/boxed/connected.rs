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

//! The destinations a boxed run connected to: each one the network took a
//! stream to, once, as `<host>:<port>`, in the order first reached. Only a
//! destination on the box's list can be reached, so there are at most as
//! many as the list has.

use alloc::vec::Vec;

use nonos_linux_policy::dests::MAX_ENTRIES;

#[derive(Default)]
pub struct Connected(Vec<Vec<u8>>);

impl Connected {
    pub fn note(&mut self, dest: &[u8]) {
        if self.0.len() < MAX_ENTRIES && !self.0.iter().any(|d| d == dest) {
            self.0.push(dest.to_vec());
        }
    }

    pub fn all(&self) -> &[Vec<u8>] {
        &self.0
    }
}
