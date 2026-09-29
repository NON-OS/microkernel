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

/* A byte cursor over one top-level selector, with the counters that bound
 * how deep and how large the parse may grow. `i` only ever rests on a
 * character boundary. */
pub(super) struct Cur<'a> {
    pub s: &'a str,
    pub i: usize,
    /* Functional pseudo-classes open around the current position. */
    pub depth: u32,
    /* Compounds parsed so far in this top-level selector. */
    pub compounds: u32,
    /* Inside a :has() argument, where another :has() is invalid. */
    pub in_has: bool,
}

impl<'a> Cur<'a> {
    pub fn new(s: &'a str) -> Self {
        Cur { s, i: 0, depth: 0, compounds: 0, in_has: false }
    }

    pub fn peek(&self) -> Option<u8> {
        self.s.as_bytes().get(self.i).copied()
    }

    pub fn at(&self, k: usize) -> Option<u8> {
        self.s.as_bytes().get(self.i + k).copied()
    }

    pub fn eat(&mut self, b: u8) -> bool {
        let hit = self.peek() == Some(b);
        self.i += hit as usize;
        hit
    }

    /* A closing parenthesis, or the end, which closes every open block. */
    pub fn close(&mut self) -> bool {
        self.eat(b')') || self.peek().is_none()
    }
}
