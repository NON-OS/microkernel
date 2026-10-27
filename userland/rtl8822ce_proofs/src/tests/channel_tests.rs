// NONOS Operating System (AGPL-3.0-or-later)
//! A retune is rtw8822c_set_channel whole: RF 0x18 with the 8822C's
//! bandwidth bits (13:12) on both paths inside the three-wire reset, the
//! spur tracking and CCK filter by channel, the MAC's 20 MHz fields, the IGI
//! toggled; a channel outside 2.4 GHz is not tuned.

use crate::model::Chip;
use crate::phy::bits::read32_mask;
use crate::phy::channel::bb::scotrk;
use crate::phy::channel::rf18::rf18_value;
use crate::phy::channel::{set_rf, Bw};
use crate::phy::rf::set_paths;
use crate::regs::Mmio;

const RF18_A: usize = 0x3C00 + (0x18 << 2);
const RF18_B: usize = 0x4C00 + (0x18 << 2);

fn tuned(ch: u8) -> Chip {
    let c = Chip::new();
    c.set32(RF18_A, 0x0008_CC00 | 0x7_33FF); // the fields set, and bits outside them
    c.set32(0x0480, 0xFF00_0000); // a sub-channel at 0x483 to clear
    set_rf(&c, ch, Bw::W20);
    c
}

#[test]
fn rf18_is_rtw88s() {
    let kept = 0x0008_CC00;
    assert_eq!(rf18_value(6, kept | 0x7_33FF), kept | 0x3006, "BW 13:12, band 2G, channel 6");
    assert_eq!(rf18_value(36, 0), 0x1_3124, "5 GHz band bits");
    assert_eq!(rf18_value(14, 0), 0x300E, "channel 14 is 2.4 GHz");
    assert_eq!(rf18_value(100, 0) & 0x6_0000, 1 << 17, "100 to 144");
    assert_eq!(rf18_value(149, 0) & 0x6_0000, 1 << 18, "149 to 177");
}

#[test]
fn both_paths_take_the_channel_inside_the_three_wire_reset() {
    let _one = super::lock();
    set_paths(2);
    let c = tuned(11);
    assert_eq!(c.read32(RF18_A) & 0xF_FFFF, 0x8_FC0B);
    assert_eq!(c.read32(RF18_B) & 0xF_FFFF, 0x8_FC0B, "path B takes path A's value");
    let off = c.at(0x1C90);
    assert_eq!((off[0] & 1 << 8, off[1] & 1 << 8), (0, 1 << 8));
    assert!(c.first(0x1C90) < c.first(RF18_A));
    assert_eq!(read32_mask(&c, 0x4C00 + (0x3F << 2), 0xF_FFFF), 0x18, "RX BB 20 MHz entry");
    assert_eq!(c.read32(0x1830) & 1 << 29, 1 << 29);
}

#[test]
fn the_baseband_follows_the_channel() {
    let _one = super::lock();
    set_paths(2);
    for (ch, sco) in [(1, 0x9AA), (10, 0x9AA), (11, 0x96A), (12, 0x96A), (13, 0x969), (14, 0x969)] {
        assert_eq!(scotrk(ch), sco);
        assert_eq!(tuned(ch).read32(0x0C30) & 0xFFF, sco, "channel {ch}");
    }
    assert_eq!(tuned(14).read32(0x1A24), 0x4962_C931, "channel 14's CCK filter");
    assert_eq!(tuned(13).read32(0x1A24), 0x3E18_FEC8);
    assert_eq!(read32_mask(&tuned(13), 0x0808, 0x70), 3);
    assert_eq!(read32_mask(&tuned(12), 0x0808, 0x70), 1);
    let c = tuned(1);
    assert_eq!(read32_mask(&c, 0x0810, 0x3FF0), 0x19B, "the 20 MHz DFIR");
    assert_eq!((c.read8(0x055C), c.read8(0x0638), c.read8(0x0483)), (80, 80, 0));
}

#[test]
fn outside_2_4_ghz_nothing_is_tuned() {
    for ch in [0, 15, 36, 149] {
        assert!(tuned(ch).writes.borrow().is_empty(), "channel {ch}");
    }
}
