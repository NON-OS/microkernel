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

use alloc::vec::Vec;

use super::color::{argb, Model};
use super::frame::Frame;
use super::plane::{zeroed, Plane};
use super::samples::idct_plane;
use super::upsample::{row as up_row, Samples};
use crate::image::types::DecodeError;

/// Transform every block into per-component sample planes at `k`/8 of the
/// natural size, then upsample and colour-convert one output row at a time
/// into `row`. `q` holds each component's quant table in natural order.
/// Returns the output size, or BadDimensions when a sample plane or row
/// cannot be allocated, so a full heap fails the decode and not the process.
pub fn emit(
    f: &Frame,
    planes: &[Plane],
    q: &[[u16; 64]; 4],
    m: Model,
    row: &mut dyn FnMut(usize, &[u32]),
) -> Result<(u32, u32), DecodeError> {
    let k = planes[0].k;
    let (ow, oh) = ((f.w * k).div_ceil(8), (f.h * k).div_ceil(8));
    let comps = &f.comps[..f.n];
    let mut samples: Vec<Vec<u8>> = Vec::new();
    for (ci, c) in comps.iter().enumerate() {
        samples.push(idct_plane(c, &planes[ci], &q[ci], k)?);
    }
    let views: Vec<Samples> = comps
        .iter()
        .zip(&samples)
        .map(|(c, px)| {
            let (fx, fy) = (f.hmax / c.h, f.vmax / c.v);
            Samples { px, stride: c.bw * k, w: ow.div_ceil(fx), h: oh.div_ceil(fy), fx, fy }
        })
        .collect();
    let mut chan: Vec<Vec<u8>> = Vec::new();
    for _ in 0..f.n {
        chan.push(zeroed(ow)?);
    }
    let mut line: Vec<u32> = zeroed(ow)?;
    for y in 0..oh {
        for (v, out) in views.iter().zip(chan.iter_mut()) {
            up_row(v, y, out);
        }
        for (x, px) in line.iter_mut().enumerate() {
            let mut s = [0u8; 4];
            for (c, ch) in s.iter_mut().zip(&chan) {
                *c = ch[x];
            }
            *px = argb(m, s);
        }
        row(y, &line);
    }
    Ok((ow as u32, oh as u32))
}
