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

//! MODE_SET: a mode VRAM holds is set and shown whole; one it does not hold
//! is refused before a register is written; one whose line pitch the
//! device pads past the mapping is refused and the old mode put back.

use super::ask::{ask, primed, words};
use super::util::up;
use crate::bench::Bench;
use crate::model::with_svga;
use nonos_gfx_wire::errno::EINVAL;
use nonos_gfx_wire::{ops, Surface};

#[test]
fn a_mode_vram_holds_is_set_and_shown_whole() {
    let mut bench = Bench::standard();
    let mut d = up(&mut bench, |_| {});
    primed(&mut d.server, 7);
    let (st, body) = ask(&mut d.server, ops::MODE_SET, &words(&[1280, 720]));
    assert_eq!(st, 0);
    let sf = Surface::decode(&body).unwrap();
    assert_eq!((sf.width, sf.height), (1280, 720));
    let m = d.server.display.mode;
    assert_eq!((m.width, m.height, m.pitch), (1280, 720, 1280 * 4));
    assert_eq!(with_svga(|s| (s.width, s.height, s.enable)), (1280, 720, 1));
    let updates = with_svga(|s| std::mem::take(&mut s.updates));
    assert_eq!(updates.last(), Some(&(0, 0, 1280, 720)), "the new mode redrawn whole");
}

#[test]
fn a_mode_vram_cannot_hold_is_refused_before_any_register_is_written() {
    let mut bench = Bench::new(64 << 10, 4 << 20);
    let mut d = up(&mut bench, |_| {});
    primed(&mut d.server, 7);
    let before = with_svga(|s| s.writes.len());
    assert_eq!(ask(&mut d.server, ops::MODE_SET, &words(&[1920, 1080])).0, EINVAL, "8 MiB in 4");
    assert_eq!(with_svga(|s| s.writes.len()), before, "no register touched");
    assert_eq!(d.server.display.mode.width, 1024, "the old mode stays");
}

#[test]
fn a_pitch_padded_past_the_mapping_puts_the_old_mode_back() {
    let mut bench = Bench::standard();
    let mut d = up(&mut bench, |_| {});
    primed(&mut d.server, 7);
    // 1920x1080 packed is 8 MiB; padded to 15680 bytes a line it is 16.1.
    with_svga(|s| s.pad = 8000);
    assert_eq!(ask(&mut d.server, ops::MODE_SET, &words(&[1920, 1080])).0, EINVAL);
    assert_eq!(with_svga(|s| (s.width, s.height)), (1024, 768), "the device back in its mode");
    assert_eq!(d.server.display.mode.width, 1024);
}
