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
//! the firmware's mode kept, the default where there is none, the line
//! pitch the device reports respected, and the framebuffer BAR mapped from
//! its start for as much as VRAM holds.

use super::util::{firmware, up};
use crate::bench::Bench;
use crate::model::{take, with_svga};

#[test]
fn the_firmware_mode_is_kept_and_the_fifo_laid_out() {
    let mut bench = Bench::standard();
    let d = up(&mut bench, |s| {
        firmware(s);
        s.pad = 256;
    });
    let svga = &d.server.display;
    let m = svga.mode;
    assert_eq!((m.width, m.height, m.pitch, m.offset), (1280, 800, 1280 * 4 + 256, 0));
    assert_eq!((svga.fifo.min, svga.fifo.max), (4096, 65536));
    assert_eq!(&bench.fifo[..4], &[4096, 65536, 4096, 4096], "MIN, MAX, NEXT_CMD, STOP");
    assert_eq!((svga.fb, svga.fb_len), (bench.vram.as_ptr() as u64, 16 << 20), "all of VRAM");
    let (regs, writes) = with_svga(|s| ((s.id, s.enable, s.config_done, s.bpp), s.writes.clone()));
    assert_eq!(regs, (0x9000_0002, 1, 1, 32));
    let config = |v| writes.iter().position(|&w| w == (20, v)).expect("CONFIG_DONE");
    assert!(config(0) < config(1), "dropped while the FIFO words change");
    assert!(d.server.slots[0].current.is_none(), "no primary until the compositor asks");
}

#[test]
fn without_a_firmware_mode_the_default_or_a_smaller_one_is_set() {
    let mut bench = Bench::standard();
    let d = up(&mut bench, |_| {});
    assert_eq!((d.server.display.mode.width, d.server.display.mode.height), (1024, 768));
    assert_eq!(with_svga(|s| (s.width, s.height, s.bpp, s.enable)), (1024, 768, 32, 1));
    drop(d);
    let _ = take();
    let mut small = Bench::new(64 << 10, 2 << 20);
    let d = up(&mut small, |_| {});
    let m = d.server.display.mode;
    assert_eq!((m.width, m.height), (800, 600), "2 MiB holds 800x600 but not 1024x768");
}

#[test]
fn an_extended_fifo_keeps_its_registers_below_the_ring() {
    let mut bench = Bench::new(256 << 10, 16 << 20);
    let d = up(&mut bench, |s| {
        s.caps = 1 << 15;
        s.mem_regs = 2048;
    });
    assert_eq!((d.server.display.fifo.min, d.server.display.fifo.max), (8192, 256 << 10));
    assert_eq!(bench.fifo[290], 0, "SVGA_FIFO_BUSY cleared");
}
