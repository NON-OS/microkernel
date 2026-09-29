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

//! A disk that fails is told apart from a file that lies.

use super::model::small_model;
use crate::{parse, ReadAt, DEFAULT};

/// A reader that fails past a point, as a disk might.
struct Failing<'a>(&'a [u8], u64);
impl ReadAt for Failing<'_> {
    fn read_at(&mut self, offset: u64, buf: &mut [u8]) -> bool {
        offset + (buf.len() as u64) <= self.1 && self.0.read_at(offset, buf)
    }
}

#[test]
fn a_reader_that_fails_is_reported_as_the_source_not_as_the_file() {
    let file = small_model();
    let got = parse(&mut Failing(&file, 10), file.len() as u64, &DEFAULT);
    assert_eq!(got, Err(crate::GgufError::Source { at: 0 }));
}
