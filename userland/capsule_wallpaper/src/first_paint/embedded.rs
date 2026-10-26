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
//! The built-in picture on a worker thread (libc's `WorkerSeat`).
//! The picture the wallpaper service links in, painted before any catalog
//! fetch returns: the catalog's default (`WALLPAPER_DEFAULT`), byte for byte.
//! wallpaper_catalog_proofs mounts this file and checks its bytes against
//! that wallpaper's pin.

pub const EMBEDDED: &[u8] =
    include_bytes!("../../../../nonos-data/wallpapers/nonos-canyon-neon.jpg");
