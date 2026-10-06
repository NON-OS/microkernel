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
use nonos_libc::{mk_surface_attach, mk_surface_release, SurfaceDescriptor};
use spin::Mutex;

use super::types::AttachedSurface;

/*
 * The last few surfaces painted into, most recent use last stamped. Apps
 * repaint in turn, so a one entry cache released and remapped a whole window
 * on every switch. An entry is released only when evicted: the kernel keeps
 * a surface's frames allocated while any attach of it is live, so a closed
 * or resized window's frames are held until its entry ages out.
 */
const CACHE_LEN: usize = 4;
static CACHE: Mutex<([Option<AttachedSurface>; CACHE_LEN], [u64; CACHE_LEN], u64)> =
    Mutex::new(([None; CACHE_LEN], [0; CACHE_LEN], 0));

pub fn attached_surface(handle: u64) -> Option<SurfaceDescriptor> {
    let mut guard = CACHE.lock();
    let (entries, stamps, clock) = &mut *guard;
    *clock = clock.wrapping_add(1);
    if let Some(i) = entries.iter().position(|e| e.map(|a| a.handle) == Some(handle)) {
        stamps[i] = *clock;
        return entries[i].map(|a| a.desc);
    }
    let mut desc = SurfaceDescriptor::default();
    let va = mk_surface_attach(handle, &mut desc);
    if va <= 0 {
        return None;
    }
    if desc.base_va == 0 {
        desc.base_va = va as u64;
    }
    let free = entries.iter().position(|e| e.is_none());
    let i = free.unwrap_or_else(|| (0..CACHE_LEN).min_by_key(|&j| stamps[j]).unwrap_or(0));
    if let Some(old) = entries[i].take() {
        let _ = mk_surface_release(old.handle);
    }
    entries[i] = Some(AttachedSurface { handle, desc });
    stamps[i] = *clock;
    Some(desc)
}
