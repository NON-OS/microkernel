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

//! What a checked header says about the model.

use crate::text::Text;
use crate::types::TYPE_SLOTS;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Summary {
    pub version: u32,
    pub tensors: u64,
    pub keys: u64,
    pub meta: Meta,
    /// Where the data section starts, from the file's start.
    pub data_start: u64,
    /// Bytes of tensor data, all of it inside the file and none shared.
    pub data_bytes: u64,
    /// Tensors of each ggml type, indexed by the type's id.
    pub by_type: [u32; TYPE_SLOTS],
}

/// The keys the layout and the model's identity depend on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Meta {
    pub alignment: u32,
    pub architecture: Option<Text>,
    pub name: Option<Text>,
    pub file_type: Option<u32>,
}
