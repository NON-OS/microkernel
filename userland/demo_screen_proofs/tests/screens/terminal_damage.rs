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


//! A key that changes only the input line repaints the input row alone, and
//! the window it leaves is pixel for pixel the one a whole repaint draws; any
//! other change asks for the whole frame.

use demo_screen_proofs::term::Terminal;
use nonos_app_skeleton::{App, InputEvent, InputKind, PaintBuffer, KEY_ENTER};

const W: u32 = 1240;
const H: u32 = 700;

fn key(code: u32) -> InputEvent {
    InputEvent { kind: InputKind::KeyDown, flags: 0, code, x: 0, y: 0, delta_x: 0, delta_y: 0, timestamp_ns: 0 }
}

fn paint(t: &mut Terminal, px: &mut [u32]) {
    t.paint(&mut PaintBuffer { pixels: px, stride_words: W, width: W, height: H });
}

fn typed(t: &mut Terminal, s: &[u8]) {
    s.iter().for_each(|&c| _ = t.on_event(key(c as u32)));
}

#[test]
fn the_row_alone_draws_what_the_whole_frame_draws() {
    nonos_libc::show(1920, 1080, "driver.virtio_gpu0");
    let (mut a, mut b) = (Terminal::new(), Terminal::new());
    let (mut pa, mut pb) = (vec![0u32; (W * H) as usize], vec![0u32; (W * H) as usize]);
    for t in [&mut a, &mut b] {
        typed(t, b"help\r");
        (0..4).for_each(|_| _ = t.on_tick());
    }
    paint(&mut a, &mut pa);
    for c in b"echo hi" {
        a.on_event(key(*c as u32));
        let row = a.take_damage().expect("only the line changed");
        assert!(row.h < H / 10, "a row, not the window: {}", row.h);
        paint(&mut a, &mut pa);
    }
    typed(&mut b, b"echo hi");
    paint(&mut b, &mut pb);
    let off = pa.iter().zip(&pb).filter(|(x, y)| x != y).count();
    assert_eq!(off, 0, "{off} pixels differ from a whole repaint");
}

#[test]
fn enter_and_other_changes_draw_the_whole_frame() {
    nonos_libc::show(1920, 1080, "driver.virtio_gpu0");
    let mut t = Terminal::new();
    let mut px = vec![0u32; (W * H) as usize];
    typed(&mut t, b"ls");
    paint(&mut t, &mut px);
    t.on_event(key(KEY_ENTER));
    assert!(t.take_damage().is_none(), "a new block: the whole frame");
    paint(&mut t, &mut px);
    typed(&mut t, b"pwd");
    paint(&mut t, &mut px);
    t.on_event(key(KEY_ENTER));
    assert!(t.take_damage().is_none(), "a second block too, the tab no longer fresh");
    paint(&mut t, &mut px);
    let mut ctrl_r = key(b'r' as u32);
    ctrl_r.flags = nonos_app_skeleton::MOD_CTRL;
    t.on_event(ctrl_r);
    assert!(t.take_damage().is_none(), "a search opens: the whole frame");
}
