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

//! What the bridge says its frames cost when a window shown from its pin
//! goes back to its own buffers: that frame was read out of the guest whole,
//! so it is counted whole with every byte read (commit/launch_clock.rs).
//! One test in a binary of its own: the frame counts are the process's.

use nonos_app_skeleton::desk;
use nonos_capsule_wayland_proofs::client::*;
use nonos_capsule_wayland_proofs::linux::call::set_clock;
use nonos_capsule_wayland_proofs::linux::guest_reads;

const W: i32 = 320;
const H: i32 = 200;
/// The frames between two of the bridge's lines on its frames.
const EVERY: u32 = 256;

fn pins(allow: bool) {
    nonos_libc::world::with(|w| w.allow_pins = allow);
}

fn commit(c: &mut Client, buffer: u32, b: (i32, i32, i32, i32)) {
    c.req(SURFACE, 1, &[A::U(buffer), A::I(0), A::I(0)]);
    c.req(SURFACE, 9, &[A::I(b.0), A::I(b.1), A::I(b.2), A::I(b.3)]);
    c.req(SURFACE, 6, &[]);
    c.events();
}

fn frame_lines() -> Vec<String> {
    let said = nonos_libc::world::with(|w| w.said.clone());
    said.iter()
        .map(|l| String::from_utf8_lossy(l).into_owned())
        .filter(|l| l.starts_with("[WAYLAND] 256 frames"))
        .collect()
}

#[test]
fn a_frame_taken_back_from_its_pin_is_counted_whole_with_what_was_read() {
    set_clock(1_000);
    desk::with(|d| (d.width, d.height) = (W as u32 + 256, H as u32 + 256));
    guest_reads::take();
    let mut c = toplevel(6);
    c.req(TOPLEVEL, 2, &[A::S("counted")]);
    c.req(SURFACE, 6, &[]);
    let serial = to(&c.events(), XDG_SURFACE).last().map(|e| e.word(0)).expect("configured");
    c.req(XDG_SURFACE, 4, &[A::U(serial)]);
    for n in 0..2 {
        c.painted(SHM, 100 + 2 * n, 101 + 2 * n, (W, H), 0xFF20_40A0 + n);
    }
    // Two frames copied whole, then a pinned one and one taken back from its
    // pin, by turns: half the frames read the window whole.
    pins(false);
    commit(&mut c, 101, (0, 0, W, H));
    commit(&mut c, 101, (0, 0, W, H));
    for i in 2..EVERY {
        let pinned = i % 2 == 0;
        pins(pinned);
        commit(&mut c, if pinned { 103 } else { 101 }, (10, 10, 40, 20));
    }
    let read = guest_reads::take().bytes;
    let whole = 2 + (EVERY - 2) / 2;
    assert!(read >= u64::from(whole) * (W * H * 4) as u64, "{read} bytes read");
    let kib = read / 1024 / u64::from(EVERY);
    let lines = frame_lines();
    assert_eq!(lines.len(), 1, "{lines:?}");
    let want = format!("[WAYLAND] {EVERY} frames, {whole} whole: ");
    assert!(lines[0].starts_with(&want), "{} is not {want}", lines[0]);
    assert!(lines[0].contains(&format!(", {kib} KiB read and ")), "{} reads {kib} KiB", lines[0]);
}
