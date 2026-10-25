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

//! A flip and a cursor update gone wrong at every step: no plane or cursor
//! reaches a frame given back, and a killed driver leaves them on frames
//! it held.

use super::kit::{rig, run, Wrong};
use super::kit_held::{hold, steps};
use crate::tests::wire_rig::Rig;
use nonos_gfx_serve::{Display, Shadow};
use nonos_gfx_wire::body::cursor::{NEW_SHAPE, VISIBLE};
use nonos_gfx_wire::{Cursor, Rect};

pub(crate) fn flush(d: &mut Rig) {
    let px = vec![0x5Au8; 1920 * 1080 * 4];
    let s = Shadow { bytes: &px, width: 1920, height: 1080, stride: 7680, cursor_offset: 0 };
    let _ = d.flush(0, &[Rect { x: 8, y: 8, width: 64, height: 32 }], &s);
}

pub(crate) fn cursor(d: &mut Rig) {
    let c = Cursor {
        flags: NEW_SHAPE | VISIBLE,
        x: 100,
        y: 80,
        hot_x: 0,
        hot_y: 0,
        width: 64,
        height: 64,
    };
    let _ = d.cursor(0, &c, &vec![0x7Fu8; 64 * 64 * 4]);
}

#[test]
fn a_flip_wrong_at_any_step_keeps_the_planes_on_frames_held() {
    for k in steps(flush) {
        for w in [Wrong::Dead(k, 0), Wrong::Dead(k, 200), Wrong::Killed(k)] {
            let (mut d, model) = rig();
            run(&model, w, || flush(&mut d));
            hold(&d, &model, w);
        }
    }
}

#[test]
fn a_cursor_update_wrong_at_any_step_keeps_the_cursor_on_its_image() {
    for k in steps(cursor) {
        for w in [Wrong::Dead(k, 0), Wrong::Killed(k)] {
            let (mut d, model) = rig();
            run(&model, w, || cursor(&mut d));
            hold(&d, &model, w);
            run(&model, Wrong::Dead(1 << 30, 0), || cursor(&mut d));
            hold(&d, &model, w);
        }
    }
}
