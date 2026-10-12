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

//! A MODE_SET and a FLUSH: the shadow copied into video memory at the
//! mode's stride, fenced before the answer.

use nonos_gfx_serve::{Display, Shadow};
use nonos_gfx_wire::Rect;

use crate::dispi::{current, DispiMode};
use crate::edid::Panel;
use crate::model::{fences, Adapter};

fn shadow(bytes: &[u8], w: u32, h: u32) -> Shadow<'_> {
    Shadow { bytes, width: w, height: h, stride: w * 4, cursor_offset: 0 }
}

fn px(v: &[u8], stride: usize, x: usize, y: usize) -> u32 {
    let i = y * stride + x * 4;
    u32::from_le_bytes(v[i..i + 4].try_into().unwrap())
}

#[test]
fn a_mode_set_programs_the_adapter_and_shows_the_whole_shadow() {
    let mut a = Adapter::new();
    let mut b = a.bga(Panel::default(), DispiMode { width: 1024, height: 768, stride: 4096 });
    let src: Vec<u8> = (0..640 * 480).flat_map(|i: u32| i.to_le_bytes()).collect();
    let before = fences();
    b.set_mode(0, 640, 480, &shadow(&src, 640, 480)).unwrap();
    let shown = DispiMode { width: 640, height: 480, stride: 2560 };
    assert_eq!((current(a.regs()), b.mode), (Some(shown), shown));
    assert_eq!(px(&a.vram, 2560, 639, 479), 640 * 480 - 1);
    assert!(fences() > before, "the copy must be fenced before the answer");
}

#[test]
fn a_flush_copies_only_its_rects_at_the_stride_scanned_out() {
    /*
     * Firmware's virtual screen here is 1280 wide for a 1024 mode; the
     * copy must land on the rows the adapter scans, not on 1024-wide ones.
     */
    let mut a = Adapter::new();
    let mut b = a.bga(Panel::default(), DispiMode { width: 1024, height: 768, stride: 5120 });
    let src = vec![0xABu8; 1024 * 768 * 4];
    let r = Rect { x: 10, y: 20, width: 4, height: 2 };
    b.flush(0, &[r], &shadow(&src, 1024, 768)).unwrap();
    assert_eq!(px(&a.vram, 5120, 10, 21), 0xABAB_ABAB);
    assert_eq!(px(&a.vram, 5120, 14, 21), 0);
    assert_eq!(px(&a.vram, 5120, 10, 22), 0);
    assert_eq!(a.vram.iter().filter(|&&v| v != 0).count(), 4 * 2 * 4);
}
