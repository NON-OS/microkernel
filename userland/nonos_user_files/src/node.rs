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

//! One file or directory, and what stat says about it.

use alloc::collections::BTreeMap;
use alloc::vec::Vec;

use crate::limits::CHUNK;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Dir,
    File,
}

#[derive(Clone, Debug)]
pub struct Node {
    pub kind: Kind,
    /// Permission bits only, 0o7777.
    pub mode: u32,
    pub size: u64,
    pub mtime: u64,
    pub ctime: u64,
    /// Its directory and its name there; the root's parent is 0.
    pub parent: u64,
    pub name: Vec<u8>,
    /// A file's chunks, one object number each, 0 for a hole. Always
    /// `chunks_for(size)` long.
    pub chunks: Vec<u64>,
    /// A directory's names.
    pub children: BTreeMap<Vec<u8>, u64>,
    /// False once unlinked or replaced while still open: kept for its
    /// handles, written to no manifest, gone at the last close.
    pub linked: bool,
    /// Handles open on it.
    pub open: u32,
}

/// What stat reports.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Attr {
    pub ino: u64,
    pub kind: Kind,
    pub mode: u32,
    pub size: u64,
    pub mtime: u64,
    pub ctime: u64,
    pub nlink: u32,
}

/// The chunks a file of `size` bytes has.
pub fn chunks_for(size: u64) -> usize {
    size.div_ceil(CHUNK as u64) as usize
}

impl Node {
    pub fn new(kind: Kind, mode: u32, parent: u64, name: &[u8], now: u64) -> Node {
        Node {
            kind,
            mode: mode & 0o7777,
            size: 0,
            mtime: now,
            ctime: now,
            parent,
            name: name.to_vec(),
            chunks: Vec::new(),
            children: BTreeMap::new(),
            linked: true,
            open: 0,
        }
    }

    pub fn attr(&self, ino: u64) -> Attr {
        // A directory says 1, as btrfs does: no count a program could take
        // for its subdirectories and skip the rest by.
        let nlink = u32::from(self.linked);
        Attr {
            ino,
            kind: self.kind,
            mode: self.mode,
            size: self.size,
            mtime: self.mtime,
            ctime: self.ctime,
            nlink,
        }
    }
}
