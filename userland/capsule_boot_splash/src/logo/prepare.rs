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

//! From MkBootLogo's two answers to a logo ready to show, or the reason
//! there is none. The record and the file must agree; anything else means
//! one of them is not what the kernel checked, and nothing is drawn.

use super::bmp::Bmp;
use super::place::{guess_mode, place};
use super::record::{Record, ON_SCREEN, TOP_DOWN};
use super::{turn, Shown};

/// Bytes of logo the splash will take, and pixels it will keep: together
/// well inside the 16 MiB heap the splash runs on.
pub const MAX_FILE: usize = 6 << 20;
pub const MAX_SHOWN: u64 = 3 << 19;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Why {
    Record,
    NotOnScreen,
    File,
    Disagree,
    Placement,
    TooLarge,
}

/// `fw` is the firmware mode when the kernel gave it; `surface` the canvas.
pub fn prepare(
    rec: &[u8],
    file: &[u8],
    fw: Option<(u32, u32)>,
    surface: (u32, u32),
) -> Result<Shown, Why> {
    let r = Record::parse(rec).ok_or(Why::Record)?;
    if r.flags & ON_SCREEN == 0 {
        return Err(Why::NotOnScreen);
    }
    if file.len() > MAX_FILE {
        return Err(Why::TooLarge);
    }
    let b = Bmp::parse(file).ok_or(Why::File)?;
    let (dw, dh) = turn::dims(b.width, b.height, r.turns);
    let sized = (r.width, r.height) == (b.width, b.height) || (r.width, r.height) == (dw, dh);
    let same = r.bpp == b.bpp && r.file_size as usize == file.len() && r.pixel_offset == b.offset;
    if !sized || !same || (r.flags & TOP_DOWN != 0) != b.top_down {
        return Err(Why::Disagree);
    }
    let mode = fw.unwrap_or_else(|| guess_mode(r.x, r.y, dw, dh, surface));
    let spot = place(r.x, r.y, dw, dh, mode, surface).ok_or(Why::Placement)?;
    if spot.w as u64 * spot.h as u64 > MAX_SHOWN {
        return Err(Why::TooLarge);
    }
    let pixels = super::resample::resample(&b, r.turns, spot);
    Ok(Shown { spot, pixels })
}
