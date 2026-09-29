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

pub const MAX_NODES: usize = 60_000;

/// Deepest an element may sit, the document being depth zero. Past it the
/// parser attaches new elements beside the current one instead of inside it,
/// as Blink does, and keeps at most this many elements open. Text may sit one
/// level lower, inside the deepest element.
pub const MAX_DEPTH: usize = 256;

/// Attributes one document may hold. Each one is two strings, so a page of
/// nothing but attributes held 17 times its own size and a few MiB of it
/// exhausted the 48 MiB heap. The largest real page seen holds 80,132.
pub const MAX_ATTRS: usize = 200_000;

/// Bytes of attribute names and values one document may hold.
pub const MAX_ATTR_BYTES: usize = 16 << 20;

/// `Dom::truncated` bits: the node cap stopped the parse.
pub const TRUNC_NODES: u8 = 1;
/// Attributes were dropped: the document budget, or a tag's own limits.
pub const TRUNC_ATTRS: u8 = 2;
/// Nesting past `MAX_DEPTH` was flattened.
pub const TRUNC_DEPTH: u8 = 4;
