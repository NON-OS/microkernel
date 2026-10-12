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

//! STATE, the hardware cursor bga does not have, and a mode the adapter
//! cannot show refused with the old one left standing.

use nonos_gfx_serve::{Display, Shadow};
use nonos_gfx_wire::body::state::SCANNING_OUT;
use nonos_gfx_wire::errno::{EINVAL, ENOSYS};
use nonos_gfx_wire::Cursor;

use crate::dispi::{current, set_mode, DispiMode};
use crate::edid::Panel;
use crate::model::Adapter;

const XGA: DispiMode = DispiMode { width: 1024, height: 768, stride: 4096 };

#[test]
fn state_is_the_mode_shown_and_cursor_is_not_offered() {
    let mut a = Adapter::new();
    let mut b = a.bga(Panel::default(), XGA);
    let s = b.state(0).unwrap();
    assert_eq!((s.width, s.height, s.flags), (1024, 768, SCANNING_OUT));
    assert_eq!(b.cursor(0, &Cursor::default(), &[]), Err(ENOSYS));
}

#[test]
fn a_mode_dispi_cannot_show_is_refused_and_the_old_one_stands() {
    /*
     * 1030 is not a multiple of 8. The server checks MODE_RANGE before the
     * driver sees a MODE_SET, but bga refuses on its own too.
     */
    let mut a = Adapter::new();
    set_mode(a.regs(), 1024, 768);
    let mut b = a.bga(Panel::default(), XGA);
    let src = vec![0xFFu8; 1030 * 768 * 4];
    let sh = Shadow { bytes: &src, width: 1030, height: 768, stride: 4120, cursor_offset: 0 };
    assert_eq!(b.set_mode(0, 1030, 768, &sh), Err(EINVAL));
    assert_eq!(current(a.regs()), Some(XGA));
    assert_eq!(b.mode, XGA);
    assert!(a.vram.iter().all(|&v| v == 0), "nothing may be copied");
}
