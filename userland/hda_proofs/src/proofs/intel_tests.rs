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
use crate::controller::intel::{
    init_link_clock, multilink, position_buffer, preferred_scf, reduce_dma_latency,
};
use crate::regs::Regs;

const BAR: u64 = 0x4000;

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

/// One run of `init_link_clock` on a link left at 6 MHz, with SPA and CPA
/// set; the link's clock select as it ends.
fn moved_clock() -> u32 {
    let bar = std::sync::Arc::new(FakeBar::new(BAR as usize));
    let ml = 0x800usize;
    bar.present16(0x14, ml as u16);
    bar.present32(ml, 0x2 << 16);
    bar.present32(ml + 0x40, 0b0111);
    bar.present32(ml + 0x44, (1 << 16) | (1 << 23));
    let regs = Regs::new(bar.base());
    assert_eq!(multilink(regs, BAR), Some(ml as u32));
    // The link reports the power state software asked for: CPA (bit 23)
    // follows SPA (bit 16). Both sit in the register's third byte, so only
    // that byte is touched and the clock select in the first is left alone.
    let _link = crate::model::live(&bar, move |b| {
        let v = b.wrote8(ml + 0x46);
        let want = if v & 1 != 0 { v | 0x80 } else { v & !0x80 };
        if v != want {
            b.present8(ml + 0x46, want);
        }
        std::thread::yield_now();
    });
    init_link_clock(regs, 0x8086, 0x3198, BAR);
    bar.wrote32(ml + 0x44) & 0xf
}

#[test]
fn a_link_left_on_the_6_mhz_clock_is_moved_to_24_mhz() {
    /*
     * The model has to see SPA drop within the millisecond the driver waits
     * for CPA; under a loaded test run its thread is sometimes not scheduled
     * in time, which proves nothing either way, so the run is repeated.
     */
    let moved = (0..20).any(|_| {
        std::thread::sleep(std::time::Duration::from_millis(10));
        moved_clock() == 2
    });
    assert!(moved, "the link clock was not moved off 6 MHz");
    assert_eq!(preferred_scf(0b1000), 3);
    assert_eq!(preferred_scf(0), 0);
}

#[test]
fn intel_playback_is_read_from_the_position_buffer_and_amd_by_lpib() {
    assert!(position_buffer(0x8086));
    assert!(!position_buffer(0x1022));
    assert!(!position_buffer(0x1002));
}

#[test]
fn gemini_lake_clears_tcsel_and_nosnoop_and_ungates_the_clock_around_reset() {
    use crate::controller::intel::pci_quirks;
    let q = pci_quirks(0x8086, 0x3198);
    assert!(q.clear_tcsel && q.clear_nosnoop && q.gate_cgctl && !q.amd_snoop);
    let skl = pci_quirks(0x8086, 0x9d70);
    assert!(skl.gate_cgctl && skl.clear_nosnoop, "Skylake-LP is in the family");
}

#[test]
fn qemu_ich9_only_has_tcsel_cleared() {
    use crate::controller::intel::pci_quirks;
    let q = pci_quirks(0x8086, 0x293e);
    assert!(q.clear_tcsel);
    assert!(!q.clear_nosnoop && !q.gate_cgctl && !q.amd_snoop, "ICH has no DEVC or CGCTL");
}

#[test]
fn amd_turns_snooping_on_and_writes_nothing_intel() {
    use crate::controller::intel::pci_quirks;
    let q = pci_quirks(0x1022, 0x15e3);
    assert!(q.amd_snoop);
    assert!(!q.clear_tcsel && !q.clear_nosnoop && !q.gate_cgctl, "AZX_DCAPS_NO_TCSEL on AMD");
    let gpu = pci_quirks(0x1002, 0xaa20);
    assert_eq!(gpu, Default::default(), "a graphics card's HDMI function is left as it is");
}

#[test]
fn a_masked_update_changes_only_the_bits_the_broker_allows() {
    use crate::controller::intel::{with_bits, AMD_ENABLE_SNOOP, AMD_SNOOP_MASK};
    assert_eq!(with_bits(0x1207, 0x0007, 0), 0x1200, "TCSEL cleared, the byte above kept");
    assert_eq!(with_bits(0x0840, 0x0040, 0), 0x0800, "MISCBDCGE off, nothing else");
    assert_eq!(with_bits(0x0800, 0x0040, 0x0040), 0x0840, "and back on");
    assert_eq!(with_bits(0xab05, AMD_SNOOP_MASK, AMD_ENABLE_SNOOP), 0xab02);
    for (cur, mask, want) in [(0xffffu16, 0x7u16, 0u16), (0x0000, 0x0800, 0), (0x1234, 0x40, 0x40)] {
        assert_eq!((with_bits(cur, mask, want) ^ cur) & !mask, 0, "a bit outside the mask moved");
    }
}
