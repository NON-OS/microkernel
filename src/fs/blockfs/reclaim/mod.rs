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

//! Reusing blocks nothing reaches.
//!
//! Every block a write made used to stay used: `alloc_block` only moved the
//! bump pointer forward, and neither a rewrite nor an unlink gave anything
//! back. A reclaim walks the tree from the root (walk.rs), and every block
//! between the header ring and the bump pointer that no node, directory
//! record, index or pointer block reaches is handed out again (pool.rs).
//!
//! A reclaim runs only when the caller has written everything it held, so
//! the disk says what the tree in memory says. A block it frees is reached by
//! nothing the disk holds then, and nothing the disk can hold later reaches
//! it unless a write made after the reclaim gave it to that thing. A cut at
//! any point therefore leaves no durable node pointing at a block another
//! file has since taken. The format is unchanged: nothing about the free
//! blocks is written, and a volume any kernel wrote is walked the same way.

mod pool;
mod runs;
mod walk;

pub(crate) use pool::{install, room, take};
pub(crate) use walk::{reach, Blocks, WalkError};
