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

/* A DEFLATE bit writer: each `put` adds `len` bits of `v`, low bit first. */
#[derive(Default)]
pub struct Bits {
    pub out: Vec<u8>,
    n: u32,
}

impl Bits {
    pub fn put(&mut self, v: u32, len: u32) {
        for k in 0..len {
            if self.n.is_multiple_of(8) {
                self.out.push(0);
            }
            *self.out.last_mut().unwrap() |= (((v >> k) & 1) as u8) << (self.n % 8);
            self.n += 1;
        }
    }
}
