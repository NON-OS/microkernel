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

//! A pack with one edit made to it, and what the parser says.

use nonos_linux_pack::{build, Pack, Refused};

use super::common::vimish;

pub fn pack() -> Vec<u8> {
    build(vimish()).expect("built")
}

pub fn refused(edit: impl FnOnce(&mut Vec<u8>)) -> Refused {
    let mut bytes = pack();
    edit(&mut bytes);
    Pack::parse(&bytes).err().expect("refused")
}

pub fn put(b: &mut [u8], at: usize, v: &[u8]) {
    b[at..at + v.len()].copy_from_slice(v);
}
