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

//! What an install produced, in the terms a person and a verifier both
//! need: the identifiers a firmware menu will show, the geometry that was
//! chosen, and where every file's bytes landed so they can be read back.

use alloc::vec::Vec;

use crate::fat32::Geometry;
use crate::gpt::Layout;
use crate::guid::Guid;

/// One file's bytes and the first sector holding them. The data is the
/// source slice the install was given, so a read-back compares against the
/// original rather than against a copy of it.
#[derive(Debug, Clone, Copy)]
pub struct FileRun<'a> {
    pub lba: u64,
    pub data: &'a [u8],
}

#[derive(Debug)]
pub struct Receipt<'a> {
    pub disk_guid: Guid,
    pub partition_guid: Guid,
    pub layout: Layout,
    pub geometry: Geometry,
    pub bytes_written: u64,
    pub files: Vec<FileRun<'a>>,
}
