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

//! FLUSH as batches against the modelled host: N rectangles cost two
//! passes of the host over the ring (all transfers, then all flushes),
//! where one command at a time cost 2N, and show the same pixels.

use nonos_gfx_wire::ops;

use super::ask::{ask, words};
use super::kit::{draw, fill, shown};
use super::rig::rig;
use crate::constants::{VG_CMD_RESOURCE_FLUSH, VG_CMD_TRANSFER_TO_HOST_2D};

const RECTS: [[u32; 4]; 5] =
    [[0, 0, 8, 8], [100, 10, 20, 5], [600, 400, 40, 80], [320, 240, 1, 1], [10, 470, 630, 10]];

fn inside(x: u32, y: u32) -> bool {
    RECTS.iter().any(|r| (r[0]..r[0] + r[2]).contains(&x) && (r[1]..r[1] + r[3]).contains(&y))
}

#[test]
fn five_rectangles_cost_two_rounds_and_show_exactly_themselves() {
    let mut r = rig();
    let s = draw(&mut r, 0, 0x0011_2233);
    fill(&mut r, 0, 0x00AB_CDEF);
    let (from, rounds) = (r.host.borrow().steps() as usize, r.host.borrow().rounds);
    let mut body = vec![s.serial, RECTS.len() as u32];
    RECTS.iter().for_each(|q| body.extend_from_slice(q));
    assert_eq!(ask(&mut r, ops::FLUSH, 0, &words(&body)).0, 0);
    let host = r.host.borrow();
    assert_eq!(host.rounds - rounds, 2, "one for the transfers, one for the flushes");
    let kinds: Vec<u32> = host.log[from..].iter().filter(|c| c.landed).map(|c| c.kind).collect();
    let mut want = vec![VG_CMD_TRANSFER_TO_HOST_2D; 5];
    want.extend([VG_CMD_RESOURCE_FLUSH; 5]);
    assert_eq!(kinds, want);
    drop(host);
    for (i, &p) in shown(&r, 0).unwrap().iter().enumerate() {
        let (x, y) = (i as u32 % 640, i as u32 / 640);
        assert_eq!(p, if inside(x, y) { 0x00AB_CDEF } else { 0x0011_2233 }, "({x}, {y})");
    }
}
