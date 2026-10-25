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

//! Adapters that fail the driver: an error naming the step, the firmware's
//! mode written back, and every map, the port grant and the claim returned,
//! in that order, by the handles' drop.

use super::util::{firmware, released, released_since};
use crate::bench::Bench;
use crate::model::take;
use crate::pool::Pool;
use crate::setup::run;

const BACK: [(&str, u64); 2] = [("pio_release", 90), ("device_release", 1)];

#[test]
fn a_host_without_version_2_is_left_untouched() {
    let mut bench = Bench::standard();
    let err = run(bench.with(|s| s.offers_id2 = false), Pool::default()).err();
    assert_eq!(err, Some("SVGA version 2 not offered"));
    assert_eq!(released(), BACK);
    assert_eq!(take().writes, vec![(0, 0x9000_0002)], "only the version asked");
}

#[test]
fn a_mode_the_host_refuses_puts_the_firmware_mode_back() {
    let mut bench = Bench::standard();
    let kept = run(
        bench.with(|s| {
            firmware(s);
            s.max = (1280, 800);
            s.refuses_modes = true;
        }),
        Pool::default(),
    );
    assert!(kept.is_ok(), "the firmware mode needs no change");
    drop(kept);
    let before = nonos_libc::calls().len();
    let mut bench = Bench::standard();
    let err = run(bench.with(|s| s.refuses_modes = true), Pool::default()).err();
    assert_eq!(err, Some("mode not taken"));
    let s = take();
    assert_eq!((s.enable, s.width, s.height, s.bpp, s.config_done), (0, 640, 480, 8, 0));
    assert_eq!(released_since(before), [vec![("mmio_unmap", 102)], BACK.to_vec()].concat());
}

#[test]
fn a_short_fifo_or_vram_is_refused() {
    let mut bench = Bench::new(12 << 10, 16 << 20);
    assert_eq!(run(bench.with(|_| {}), Pool::default()).err(), Some("FIFO too small for a ring"));
    assert_eq!(released(), [vec![("mmio_unmap", 102)], BACK.to_vec()].concat(), "the FIFO's map");
    let before = nonos_libc::calls().len();
    let mut bench = Bench::standard();
    let err = run(bench.with(|s| s.fb_offset = 15 << 20), Pool::default()).err();
    assert_eq!(err, Some("framebuffer past VRAM"));
    let maps = vec![("mmio_unmap", 101), ("mmio_unmap", 102)];
    assert_eq!(released_since(before), [maps, BACK.to_vec()].concat(), "both maps, the last first");
}
