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

//! The display box of a source, noted before it is ingested.

use super::Store;

/* Record the box a source will be drawn into before ingest, so a vector image
 * rasterizes at its display size instead of upscaling a default raster. The
 * on-device path does this through the fetch queue; the host render harness has
 * no queue and calls this directly. */
pub fn note_size(store: &mut Store, url: &str, w: u32, h: u32) {
    store.mark_pending(url);
    store.note_hint(url, w, h);
}
