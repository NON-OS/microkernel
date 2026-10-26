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

//! OP_GET_LOGO's reply: one byte, the
//! side of a square RGBA raster, then its side * side * 4 bytes, nothing
//! after. A side of 0 and no bytes is a listing with no logo to show.

use alloc::vec::Vec;

/// The largest side served: the store draws a logo at its own pixels, and
/// a card holds no more.
pub const LOGO_MAX: usize = 48;

pub struct Logo {
    pub side: u32,
    pub rgba: Vec<u8>,
}

/// The side of a square RGBA raster `len` bytes long, if it is one no
/// larger than LOGO_MAX.
pub fn logo_side(len: usize) -> Option<usize> {
    (1..=LOGO_MAX).find(|s| s.checked_mul(*s).and_then(|a| a.checked_mul(4)) == Some(len))
}

/// The reply for `rgba`, or for no logo when it is None; None when the
/// raster is not one logo_side accepts.
pub fn logo_body(rgba: Option<&[u8]>) -> Option<Vec<u8>> {
    let Some(rgba) = rgba else {
        return Some(alloc::vec![0]);
    };
    let side = u8::try_from(logo_side(rgba.len())?).ok()?;
    let mut out = Vec::with_capacity(rgba.len().saturating_add(1));
    out.push(side);
    out.extend_from_slice(rgba);
    Some(out)
}

/// Some(None) for no logo, Some(logo) for one, None for a body that is
/// neither.
pub fn parse_logo(body: &[u8]) -> Option<Option<Logo>> {
    let (&side, rgba) = body.split_first()?;
    if side == 0 {
        return rgba.is_empty().then_some(None);
    }
    (logo_side(rgba.len())? == usize::from(side))
        .then(|| Some(Logo { side: u32::from(side), rgba: rgba.to_vec() }))
}
