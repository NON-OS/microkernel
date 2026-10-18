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

//! A present names the one resource this driver has and, for a transfer,
//! where in the surface its rectangle starts: on a pixel, at the
//! rectangle's own place. Another resource is E_INVAL even from the
//! compositor that holds the primary, and so is an offset between pixels or
//! one pixel past; neither moves a pixel.

use super::frames::{ask, fields, transfer_body};
use super::model::{paint, served, take, COMPOSITOR};
use crate::constants::CLEAR_COLOR;
use crate::serve::{E_INVAL, OP_FLUSH, OP_TRANSFER_TO_HOST};

const W: u32 = 64;
const H: u32 = 48;

#[test]
fn a_resource_other_than_the_primary_is_e_inval_from_its_holder() {
    let mut s = served(W as u16, H as u16);
    take(&mut s);
    paint(&s);
    for resource in [0, 2, u32::MAX] {
        let flush = fields(&[resource, 0, 0, W, H]);
        assert_eq!(ask(&mut s.d, COMPOSITOR, OP_FLUSH, &flush).0, E_INVAL, "flush of {resource}");
        let mut transfer = fields(&[resource, 0, 0, W, H, 0]);
        transfer.extend_from_slice(&0u64.to_le_bytes());
        let status = ask(&mut s.d, COMPOSITOR, OP_TRANSFER_TO_HOST, &transfer).0;
        assert_eq!(status, E_INVAL, "transfer of {resource}");
    }
    let untouched = (0..W * H).all(|i| s.a.fb.wrote32(i as usize * 4) == CLEAR_COLOR);
    assert!(untouched, "a present of another resource moved a pixel");
}

#[test]
fn a_transfer_offset_between_pixels_or_past_its_rectangle_is_e_inval() {
    let mut s = served(W as u16, H as u16);
    take(&mut s);
    let last = (W * (H - 1) * 4) as u64;
    let mut transfer = |y, h, offset| {
        ask(&mut s.d, COMPOSITOR, OP_TRANSFER_TO_HOST, &transfer_body(0, y, W, h, offset)).0
    };
    assert_eq!(transfer(H - 1, 1, last), 0, "the last line refused at its own place");
    for offset in [last + 1, last + 2, last + 3, last + 4] {
        assert_eq!(transfer(H - 1, 1, offset), E_INVAL, "the last line at {offset}");
    }
    for offset in [1, 2, 3, u64::MAX - 8] {
        assert_eq!(transfer(0, H, offset), E_INVAL, "the whole mode at {offset}");
    }
}
