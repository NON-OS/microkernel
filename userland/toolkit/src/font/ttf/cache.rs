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

//! The rasterized-glyph cache. Outlining a glyph with ab_glyph is expensive,
//! so a full screen of text every repaint would be slow. Each (face, glyph,
//! size, subpixel phase) is rasterized once into a coverage bitmap and reused;
//! the colour is applied at blit time, so the same glyph serves every colour.
//! Capsules are single threaded, so the lock never actually contends.

use spin::{Mutex, MutexGuard};

use super::store::Store;

static CACHE: Mutex<Store> = Mutex::new(Store::new());

pub(super) fn lock() -> MutexGuard<'static, Store> {
    CACHE.lock()
}

/// Drop every cached glyph. The cache knows a face by the address of its
/// data, so whoever frees face data calls this before that memory can hold
/// another face: the page font registry does on every navigation.
pub fn clear_glyph_cache() {
    CACHE.lock().clear();
}

/// Bytes the glyph cache holds against `GLYPH_CACHE_BUDGET`.
pub fn glyph_cache_bytes() -> usize {
    CACHE.lock().bytes
}
