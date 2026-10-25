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

//! A good adapter: version 2 agreed, the FIFO laid out as Linux lays it,
//! the firmware's mode kept, the default where there is none, and the
//! framebuffer line pitch the device reports respected.

use super::util::{firmware, up};
use crate::bench::Bench;
use crate::model::take;

#[test]
fn the_firmware_mode_is_kept_and_the_fifo_laid_out() {
    let mut bench = Bench::standard();
    let d = up(&mut bench, |s| {
        firmware(s);
        s.pad = 256;
    });
    assert_eq!((d.fb.width, d.fb.height, d.fb.stride), (1280, 800, 1280 * 4 + 256));
    assert_eq!(
        (d.surface.width, d.surface.stride, d.fifo.min, d.fifo.max),
        (1280, 5120, 4096, 65536)
    );
    assert_eq!(&bench.fifo[..4], &[4096, 65536, 4096, 4096], "MIN, MAX, NEXT_CMD, STOP");
    let s = take();
    assert_eq!((s.id, s.enable, s.config_done, s.bpp), (0x9000_0002, 1, 1, 32));
    let config = |v| s.writes.iter().position(|&w| w == (20, v)).expect("CONFIG_DONE");
    assert!(config(0) < config(1), "dropped while the FIFO words change");
    let (surfaces, _) = nonos_libc::surfaces();
    assert_eq!((surfaces[0].width, surfaces[0].stride, surfaces[0].format), (1280, 5120, 1));
    assert_eq!((d.sid, d.handle), (3, 0), "registered, and shared with no one yet");
    assert!(nonos_libc::shares().is_empty(), "no handle of its own (K20)");
}

#[test]
fn without_a_firmware_mode_the_default_or_a_smaller_one_is_set() {
    let mut bench = Bench::standard();
    let d = up(&mut bench, |_| {});
    assert_eq!((d.fb.width, d.fb.height), (1024, 768));
    let s = take();
    assert_eq!((s.width, s.height, s.bpp, s.enable), (1024, 768, 32, 1));
    let mut small = Bench::new(64 << 10, 2 << 20);
    let d = up(&mut small, |_| {});
    assert_eq!((d.fb.width, d.fb.height), (800, 600), "2 MiB holds 800x600 but not 1024x768");
}

#[test]
fn an_extended_fifo_keeps_its_registers_below_the_ring() {
    let mut bench = Bench::new(256 << 10, 16 << 20);
    let d = up(&mut bench, |s| {
        s.caps = 1 << 15;
        s.mem_regs = 2048;
    });
    assert_eq!((d.fifo.min, d.fifo.max), (8192, 256 << 10));
    assert_eq!(bench.fifo[290], 0, "SVGA_FIFO_BUSY cleared");
}
