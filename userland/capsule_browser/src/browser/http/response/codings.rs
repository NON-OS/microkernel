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

use super::bytes::{list, trim};

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Coding {
    Gzip,
    Deflate,
    Chunked,
}

/* Most codings one header kind may stack. */
pub const MAX_CODINGS: usize = 4;

/* The codings of every Content-Encoding (or Transfer-Encoding) field, in
the order they were applied, identity dropped (RFC 9110 8.4, RFC 9112
6.1: repeated fields combine into one list). A coding this browser
cannot undo, or more than `MAX_CODINGS`, marks the list undecodable. */
#[derive(Clone, Copy)]
pub struct Codings {
    list: [Coding; MAX_CODINGS],
    len: usize,
    pub undecodable: bool,
    pub present: bool,
}

impl Codings {
    pub const NONE: Codings =
        Codings { list: [Coding::Gzip; MAX_CODINGS], len: 0, undecodable: false, present: false };

    pub fn push_list(&mut self, value: &[u8]) {
        for item in list(value) {
            let name = trim(item.split(|&c| c == b';').next().unwrap_or(item));
            let is = |s: &[u8]| name.eq_ignore_ascii_case(s);
            let c = if is(b"gzip") || is(b"x-gzip") {
                Some(Coding::Gzip)
            } else if is(b"deflate") {
                Some(Coding::Deflate)
            } else if is(b"chunked") {
                Some(Coding::Chunked)
            } else if is(b"identity") {
                continue;
            } else {
                None
            };
            self.present = true;
            match c {
                Some(c) if self.len < MAX_CODINGS => {
                    self.list[self.len] = c;
                    self.len += 1;
                }
                _ => self.undecodable = true,
            }
        }
    }

    pub fn as_slice(&self) -> &[Coding] {
        &self.list[..self.len]
    }
}
