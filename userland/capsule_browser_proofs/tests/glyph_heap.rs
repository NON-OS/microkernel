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

//! Heap proofs for text drawing, under an allocator that counts what each
//! test thread allocates: a hostile face's oversized glyphs cost nothing,
//! and warm page text neither rasterises nor reparses a face.

#[path = "glyph_heap/cap.rs"]
mod cap;
#[path = "glyph_heap/counting.rs"]
mod counting;
#[path = "glyph_heap/held.rs"]
mod held;
#[path = "glyph_cache/serial.rs"]
mod serial;
#[path = "glyph_heap/sfnt.rs"]
mod sfnt;
#[path = "glyph_heap/warm.rs"]
mod warm;

#[global_allocator]
static COUNTING: counting::Counting = counting::Counting;
