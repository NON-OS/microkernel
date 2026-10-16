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
//! The Intel controller registers Linux touches beyond the specification.

use nonos_devmodel::FakeBar;

use crate::constants::VS_EM4L;
use crate::controller::intel::{multilink, position_buffer, reduce_dma_latency};
use crate::regs::Regs;

pub const BAR: u64 = 0x4000;

#[test]
fn only_apollo_lake_has_its_dma_fifo_threshold_lowered() {
    let bar = FakeBar::new(BAR as usize);
    bar.present32(VS_EM4L as usize, 0xffff_ffff);
    reduce_dma_latency(Regs::new(bar.base()), 0x8086, 0x3198, BAR);
    assert_eq!(bar.wrote32(VS_EM4L as usize), 0xffff_ffff, "Gemini Lake is not Apollo Lake");
    reduce_dma_latency(Regs::new(bar.base()), 0x8086, 0x5a98, BAR);
    assert_eq!(bar.wrote32(VS_EM4L as usize), 0x3 << 20);
}

#[test]
fn a_register_beyond_a_small_bar_is_not_touched() {
    let bar = FakeBar::new(0x1000);
    reduce_dma_latency(Regs::new(bar.base()), 0x8086, 0x5a98, 0x1000);
}

/// The multi-link capability header the walk looks for.
const ML_HEADER: u32 = 0x2 << 16;

#[test]
fn a_multi_link_pointer_not_4_byte_aligned_ends_the_walk() {
    for llch in [0x801u16, 0x802, 0x803] {
        let bar = FakeBar::new(BAR as usize);
        bar.present16(0x14, llch);
        bar.present32(llch as usize, ML_HEADER);
        assert_eq!(multilink(Regs::new(bar.base()), BAR), None, "{llch:#x}");
    }
}

#[test]
fn a_multi_link_walk_ends_past_the_window_and_on_a_loop() {
    let bar = FakeBar::new(BAR as usize);
    let regs = Regs::new(bar.base());
    // Its link control register would end four bytes past the window.
    let last = BAR as usize - 0x44;
    bar.present16(0x14, last as u16);
    bar.present32(last, ML_HEADER);
    assert_eq!(multilink(regs, BAR), None);
    // A capability that names itself as the next.
    bar.present16(0x14, 0x800);
    bar.present32(0x800, 0x800);
    assert_eq!(multilink(regs, BAR), None);
}

#[test]
fn intel_playback_is_read_from_the_position_buffer_and_amd_by_lpib() {
    assert!(position_buffer(0x8086));
    assert!(!position_buffer(0x1022));
    assert!(!position_buffer(0x1002));
}
