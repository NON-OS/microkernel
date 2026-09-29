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

use super::bits::Bools;
use super::fields::{Segmented, Strength};

/// The filter header (RFC 6386 9.4) turned into the edge limits each
/// segment uses, for whole-block and 4x4-predicted macroblocks. Returns
/// the filter kind: 0 none, 1 simple, 2 normal.
pub(super) fn filter_strengths(br: &mut Bools, seg: Segmented) -> (u8, [[Strength; 2]; 4]) {
    let simple = br.bit(128);
    let level = br.literal(6) as i32;
    let sharpness = br.literal(3) as i32;
    let (mut ref0, mut mode0) = (0, 0);
    let deltas = br.bit(128);
    if deltas && br.bit(128) {
        for i in 0..8 {
            if br.bit(128) {
                let v = br.signed(6);
                match i {
                    0 => ref0 = v,
                    4 => mode0 = v,
                    _ => {}
                }
            }
        }
    }
    let kind = if level == 0 { 0 } else { 2 - simple as u8 };
    let mut out = [[Strength::default(); 2]; 4];
    for (s, row) in out.iter_mut().enumerate() {
        let base = match seg {
            Some((true, v)) => v[s],
            Some((false, v)) => v[s] + level,
            None => level,
        };
        for (i4x4, st) in row.iter_mut().enumerate() {
            let mut lv = base;
            if deltas {
                lv += ref0 + if i4x4 == 1 { mode0 } else { 0 };
            }
            let lv = lv.clamp(0, 63);
            if lv > 0 {
                let mut il = lv;
                if sharpness > 0 {
                    il >>= if sharpness > 4 { 2 } else { 1 };
                    il = il.min(9 - sharpness);
                }
                let il = il.max(1);
                let hev = if lv >= 40 { 2 } else { i32::from(lv >= 15) };
                *st = Strength { limit: 2 * lv + il, ilevel: il, hev, inner: i4x4 == 1 };
            } else {
                *st = Strength { inner: i4x4 == 1, ..Strength::default() };
            }
        }
    }
    (kind, out)
}
