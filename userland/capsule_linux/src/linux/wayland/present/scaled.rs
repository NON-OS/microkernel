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

//! A committed buffer at the size it is shown (scale_math.rs): the window's
//! at the ratio its toplevel and the display give, kept for the pointer and
//! the layers, and a subsurface's or a popup's at the window's ratio.

use alloc::vec::Vec;

use nonos_app_skeleton::runner::chrome::quarters_for;

use crate::linux::guest::MemfdSpan;

use super::super::{frame_len::MAX_FRAME, reshape::Shape, scene::Scene};
use super::crop::{corner, crop, crop_dest};
use super::resample::resample;
use super::scale_math::{display, shown, window_ratio, Scale};
use super::source::{cropped, source, take, Source};

/// The display's scale in quarters, as the frame reads it.
pub fn quarters() -> u32 {
    let (w, h) = super::super::output::screen();
    quarters_for(w, h)
}

/// `buffer`, committed on the window's `surface`, at the size it is shown;
/// the window's ratio is noted for what else is placed on it.
pub fn window(
    scene: &mut Scene,
    spans: &[MemfdSpan],
    surface: u32,
    buffer: u32,
) -> Result<Source, &'static [u8]> {
    super::frame::now::note_scale(scene);
    let s = scale_of(scene, surface);
    scene.win.frame.ratio = window_ratio(s, display(scene.win.frame.quarters));
    let (src, s) = geometry_box(scene, surface, s, source(scene, spans, buffer)?);
    Ok(at(src, s, scene.win.frame.ratio))
}

/// `buffer`, committed on a popup's `surface`, shown in a window of its own:
/// its window geometry's box alone, as a toplevel's, at the window's ratio.
/// The positioner placed that box, so a shadow drawn round it is not shown
/// past it and the menu sits where it was placed.
pub fn popup(
    scene: &Scene,
    spans: &[MemfdSpan],
    surface: u32,
    buffer: u32,
) -> Result<Source, &'static [u8]> {
    let s = scale_of(scene, surface);
    let (src, s) = geometry_box(scene, surface, s, source(scene, spans, buffer)?);
    Ok(at(src, s, scene.win.frame.ratio))
}

/// `src` cut to the box `surface`'s window geometry names, when it names
/// one, and its scale with a viewport's destination taken to that box's
/// logical size, which is then the size it is shown at.
fn geometry_box(scene: &Scene, surface: u32, s: Scale, src: Source) -> (Source, Scale) {
    let geometry = scene.surfaces.iter().find(|x| x.id == surface).and_then(|x| x.geometry);
    let buffer = (src.shape.0, src.shape.1);
    if let Some(dest) = s.dest {
        // A viewport's geometry is in its destination's pixels.
        return match crop_dest(geometry, dest, buffer) {
            Some((b, size)) => (cropped(src, b, corner(geometry)), Scale { dest: Some(size), ..s }),
            None => (src, s),
        };
    }
    let k = s.buffer.max(1);
    match crop(geometry, s.buffer, buffer) {
        Some(b) => (cropped(src, b, ((b.0 / k) as i32, (b.1 / k) as i32)), s),
        None => (src, s),
    }
}

/// `buffer`, committed on `surface`, a layer of the window.
pub fn layer(
    scene: &Scene,
    spans: &[MemfdSpan],
    surface: u32,
    buffer: u32,
) -> Result<Source, &'static [u8]> {
    source(scene, spans, buffer).map(|src| at(src, scale_of(scene, surface), scene.win.frame.ratio))
}

fn scale_of(scene: &Scene, surface: u32) -> Scale {
    scene.surfaces.iter().find(|s| s.id == surface).map_or(Scale::default(), |s| s.scale)
}

/// `src` at ratio `e`, or as it is when that is its size or more than a frame.
fn at(src: Source, s: Scale, e: u32) -> Source {
    let (w, h, _) = src.shape;
    let (sw, sh) = shown(s, (w, h), e);
    let bytes = u64::from(sw) * u64::from(sh) * 4;
    if (sw, sh) == (w, h) || bytes > MAX_FRAME {
        return src;
    }
    let raw = Some((src.shape, src.bytes));
    Source { shape: (sw, sh, sw * 4), bytes: bytes as usize, raw, ..src }
}

/// The buffer read whole, then brought to the size it is shown at.
pub fn take_scaled(pid: u32, src: &Source, (shape, bytes): (Shape, usize), out: &mut [u8]) -> bool {
    let mut px = Vec::new();
    if px.try_reserve_exact(bytes).is_err() {
        return false;
    }
    px.resize(bytes, 0);
    let plain = Source { shape, bytes, raw: None, ..*src };
    let (w, h) = (src.shape.0, src.shape.1);
    let done = take(pid, &plain, &mut px) && resample(&px, shape, out, (w, h));
    if done {
        super::super::commit::launch_clock::laid(super::super::frame_cost::box_bytes(w, h));
    }
    done
}
