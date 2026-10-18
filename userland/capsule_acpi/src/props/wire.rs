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

//! A device's _DSD tree as the PROPERTIES op (12) carries it: pure and
//! self-contained, so a driver can mount this file to read what it gets.
//!
//! tree := name, property count u8, properties, child count u8, children
//! property := name, value
//! name := length u8, bytes
//! value := 1 u64 | 2 length u16, bytes | 3 namespace path (a name) |
//!          4 count u8, values
//!
//! Little-endian. Children nest `MAX_LEVELS` deep, packages `MAX_DEPTH`;
//! a tree that would pass `MAX_WIRE` bytes, or any count past `MAX`, is
//! not encoded.

use alloc::vec::Vec;

pub const MAX_WIRE: usize = 16 * 1024;
pub const MAX: usize = 64;
pub const MAX_DEPTH: u32 = 4;
pub const MAX_LEVELS: u32 = 5;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Val {
    Int(u64),
    Str(Vec<u8>),
    Ref(Vec<u8>),
    Pkg(Vec<Val>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Prop {
    pub name: Vec<u8>,
    pub val: Val,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Tree {
    pub name: Vec<u8>,
    pub props: Vec<Prop>,
    pub children: Vec<Tree>,
}
