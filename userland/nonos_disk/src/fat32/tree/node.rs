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

//! A directory or a file to write. A name that fits 8.3 in one case per
//! part is written as a short name; any other is written as a long name
//! beside a generated alias. One that is not a valid FAT name at all is
//! refused at write time, not guessed at.

use alloc::vec::Vec;

pub enum Node<'a> {
    Dir { name: &'a str, children: Vec<Node<'a>> },
    File { name: &'a str, data: &'a [u8] },
}

impl<'a> Node<'a> {
    pub fn dir(name: &'a str, children: Vec<Node<'a>>) -> Node<'a> {
        Node::Dir { name, children }
    }

    pub fn file(name: &'a str, data: &'a [u8]) -> Node<'a> {
        Node::File { name, data }
    }

    pub fn name(&self) -> &'a str {
        match self {
            Node::Dir { name, .. } | Node::File { name, .. } => name,
        }
    }
}
