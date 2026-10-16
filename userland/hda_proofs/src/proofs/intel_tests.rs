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
use crate::controller::intel::{position_buffer, reduce_dma_latency};
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

#[test]
fn intel_playback_is_read_from_the_position_buffer_and_amd_by_lpib() {
    assert!(position_buffer(0x8086));
    assert!(!position_buffer(0x1022));
    assert!(!position_buffer(0x1002));
}
