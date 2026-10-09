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

//! The desktop peers an app draws through, found when it is first opened.
//!
//! A base app is spawned at boot with the rest of the desktop, and the kernel
//! spawns the image and video players before the toolkit they draw through
//! (`desktop_services::spawn`). Looked up at start, the toolkit could still be
//! missing when the bounded wait in `require_peers` ran out, and the app
//! exited with code 2 and stayed gone until the next boot. An app needs no
//! peer while it idles, so they are looked up when the shell first asks it to
//! open, by when the desktop is up, and kept. A lookup that still fails
//! leaves the cache empty, so the next open asks again.

/// The cached peers, looked up through `lookup` when none are cached yet.
pub(super) fn open_peers<P>(
    cache: &mut Option<P>,
    lookup: impl FnOnce() -> Option<P>,
) -> Option<&P> {
    if cache.is_none() {
        *cache = lookup();
    }
    cache.as_ref()
}
