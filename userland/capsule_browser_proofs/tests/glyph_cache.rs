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

//! Proofs for the toolkit's glyph cache and integer glyph blend: the cache,
//! blend and blit sources compiled here unchanged, and the page and chrome
//! text paths driven through the public API the browser calls.

extern crate alloc;

#[path = "glyph_cache/budget.rs"]
mod budget;
#[path = "glyph_cache/chrome.rs"]
mod chrome;
#[path = "glyph_cache/faces.rs"]
mod faces;
#[path = "glyph_cache/serial.rs"]
mod serial;
#[path = "glyph_cache/ttf.rs"]
mod ttf;
