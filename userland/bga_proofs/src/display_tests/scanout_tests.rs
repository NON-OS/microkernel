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

//! set_scanout: the adapter's one scanout shows the whole mode, so that is
//! the one rectangle it takes, and its answer is read off the adapter.

use super::frames::{ask, fields, scanout_body};
use super::model::{served, take, COMPOSITOR};
use super::rects::{H, INSIDE, OUTSIDE, W};
use crate::constants::{DISPI_INDEX_ENABLE, DISPI_INDEX_XRES, DISPI_INDEX_Y_OFFSET};
use crate::dispi::dispi_off;
use crate::serve::{E_DEVICE, E_INVAL, OP_SET_SCANOUT};

#[test]
fn set_scanout_takes_the_whole_mode_on_scanout_0_and_nothing_else() {
    let mut s = served(W as u16, H as u16);
    take(&mut s);
    assert_eq!(ask(&mut s.d, COMPOSITOR, OP_SET_SCANOUT, &scanout_body(0, 0, W, H)).0, 0);
    for (x, y, w, h) in INSIDE.into_iter().skip(1).chain(OUTSIDE) {
        let status = ask(&mut s.d, COMPOSITOR, OP_SET_SCANOUT, &scanout_body(x, y, w, h)).0;
        assert_eq!(status, E_INVAL, "({x}, {y}) {w}x{h} shown");
    }
    for (scanout, resource) in [(1, 1), (u32::MAX, 1), (0, 0), (0, 2), (0, u32::MAX)] {
        let body = fields(&[scanout, resource, 0, 0, W, H]);
        let status = ask(&mut s.d, COMPOSITOR, OP_SET_SCANOUT, &body).0;
        assert_eq!(status, E_INVAL, "scanout {scanout}, resource {resource} taken");
    }
}

#[test]
fn set_scanout_is_e_device_once_the_adapter_no_longer_shows_the_mode() {
    let mut s = served(W as u16, H as u16);
    take(&mut s);
    let whole = scanout_body(0, 0, W, H);
    let at = |index| dispi_off(index) as usize;
    for (index, value) in
        [(DISPI_INDEX_XRES, 800), (DISPI_INDEX_Y_OFFSET, 8), (DISPI_INDEX_ENABLE, 0)]
    {
        let held = s.a.regs.wrote16(at(index));
        s.a.regs.present16(at(index), value);
        assert_eq!(ask(&mut s.d, COMPOSITOR, OP_SET_SCANOUT, &whole).0, E_DEVICE, "{index}");
        s.a.regs.present16(at(index), held);
        assert_eq!(ask(&mut s.d, COMPOSITOR, OP_SET_SCANOUT, &whole).0, 0, "{index} put back");
    }
}
