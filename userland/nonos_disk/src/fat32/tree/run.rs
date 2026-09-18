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

//! What placement produces: every directory and file as one run of
//! clusters, and each directory's slots pointing at the runs they name.

use alloc::vec::Vec;

/// One 32-byte slot in a placed directory.
pub struct Entry<'a> {
    pub name: &'a str,
    pub is_dir: bool,
    /// Index into `Placed::runs`.
    pub run: usize,
    pub size: u32,
}

pub enum Content<'a> {
    Dir(Vec<Entry<'a>>),
    File(&'a [u8]),
}

/// A contiguous run of clusters holding one directory or one file. An
/// empty file has no run and `first_cluster` zero, which is how FAT spells
/// "no data".
pub struct Run<'a> {
    pub first_cluster: u32,
    pub clusters: u32,
    pub content: Content<'a>,
    /// The parent directory's run, for the `..` slot. The root has none.
    pub parent: Option<usize>,
}

pub struct Placed<'a> {
    /// Run 0 is always the root directory at cluster 2.
    pub runs: Vec<Run<'a>>,
    pub data_clusters_needed: u64,
}
