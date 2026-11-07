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

//! The window as it reaches the compositor: the surface the server
//! registered last, read from where its descriptor points, written as a
//! PPM; and the scene, said line by line.

use std::io::Write;

use crate::linux::guest::Guest;

/// The pixels of the window's surface, or None before one is shown.
pub fn pixels(guest: &Guest) -> Option<(u32, u32, Vec<u32>)> {
    surface_pixels(guest.scene.win.out?)
}

/// The pixels of the surface registered as `handle`: a window shown, or
/// one kept behind it.
pub fn surface_pixels(handle: u64) -> Option<(u32, u32, Vec<u32>)> {
    let d = nonos_libc::world::surface(handle)?;
    let (w, h, stride) = (d.width as usize, d.height as usize, d.stride as usize);
    if d.byte_len < (stride * h) as u64 || stride < w * 4 {
        return None;
    }
    // SAFETY: the descriptor points at the scene's own pixels, which the
    // guest holds, `byte_len` bytes of them (scene_pixels.rs).
    let bytes = unsafe { core::slice::from_raw_parts(d.base_va as *const u8, stride * h) };
    let px = (0..h)
        .flat_map(|y| (0..w).map(move |x| (y, x)))
        .map(|(y, x)| {
            let at = y * stride + x * 4;
            u32::from_le_bytes([bytes[at], bytes[at + 1], bytes[at + 2], bytes[at + 3]])
        })
        .collect();
    Some((d.width, d.height, px))
}

pub fn dump(guest: &Guest, path: &str) {
    let Some((w, h, px)) = pixels(guest) else {
        return eprintln!("[host] dump: no window shown");
    };
    let mut out = format!("P6\n{w} {h}\n255\n").into_bytes();
    for p in px {
        out.extend_from_slice(&[(p >> 16) as u8, (p >> 8) as u8, p as u8]);
    }
    match std::fs::File::create(path).and_then(|mut f| f.write_all(&out)) {
        Ok(()) => eprintln!("[host] dump: {w}x{h} to {path}"),
        Err(e) => eprintln!("[host] dump {path}: {e}"),
    }
}

pub fn state(guest: &Guest) {
    let s = &guest.scene;
    eprintln!(
        "[host] state: {} surfaces, {} buffers, {} pools, shown {:?}, at {:?}, shape {:?}, popups {}, pointer_on {:?}, keyboard_on {:?}, focused {}",
        s.surfaces.len(),
        s.buffers.len(),
        s.pools.len(),
        s.win.shown.map(|w| (w.surface, w.toplevel)),
        s.win.at,
        s.win.shape,
        s.popups.list.len(),
        s.pointer_on,
        s.keyboard_on,
        s.win.focused,
    );
}
