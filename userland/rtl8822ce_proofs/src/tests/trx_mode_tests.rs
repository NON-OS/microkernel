// NONOS Operating System (AGPL-3.0-or-later)
//! rtw8822c_config_trx_mode: both paths' TX codes, receive on the paths
//! asked, CCK and 1-stream OFDM sent from path A, the BB reset after each
//! transmit setting, and the IGI toggled back to where it was.

use crate::model::Chip;
use crate::phy::bits::read32_mask;
use crate::phy::trx::{config, BB_PATH_A, BB_PATH_AB};
use crate::regs::Mmio;

fn run(tx: u8, rx: u8) -> Chip {
    let c = Chip::new();
    c.set32(0x1D70, 0x2626);
    config(&c, tx, rx);
    c
}

#[test]
fn two_paths_receive_and_path_a_sends() {
    let c = run(BB_PATH_AB, BB_PATH_AB);
    assert_eq!(c.read32(0x1800) & 0xF_FFFF, 0x33312);
    assert_eq!(c.read32(0x4100) & 0xF_FFFF, 0x33312);
    assert_eq!(read32_mask(&c, 0x1A2C, 0x0066_0000), 0x11, "CCK on both");
    assert_eq!(read32_mask(&c, 0x1A04, 0x0F00_0000), 0x1, "CCK RX A and B");
    assert_eq!(read32_mask(&c, 0x1A04, 0xF000_0000), 0x8, "CCK TX A");
    assert_eq!(read32_mask(&c, 0x1D30, 0x300), 1);
    assert_eq!(read32_mask(&c, 0x0C38, 1 << 24), 1, "MRC on");
    assert_eq!(read32_mask(&c, 0x0824, 0x0F0F_0000), 0x0303);
    assert_eq!(c.read32(0x0820) & 0xFF, 0x31, "1SS on A of AB");
    assert_eq!(c.read32(0x1E2C) & 0xFFFF, 0x0400);
}

#[test]
fn one_path_is_path_a_alone() {
    let c = run(BB_PATH_A, BB_PATH_A);
    assert_eq!(c.read32(0x4100) & 0xF_FFFF, 0x11111);
    assert_eq!(read32_mask(&c, 0x1A04, 0x0F00_0000), 0x0);
    assert_eq!(read32_mask(&c, 0x0C38, 1 << 24), 0, "MRC off");
    assert_eq!(read32_mask(&c, 0x0824, 0x0F0F_0000), 0x0101);
    assert_eq!(c.read32(0x0820) & 0xFF, 0x11);
}

#[test]
fn three_bb_resets_and_the_igi_comes_back() {
    let c = run(BB_PATH_AB, BB_PATH_AB);
    assert_eq!(c.at(0x0002).len(), 9, "three pulses");
    assert_eq!(c.read32(0x1D70) & 0x7F7F, 0x2626);
    let igi = c.at(0x1D70);
    assert_eq!(igi.len(), 4);
    assert_eq!(igi[0] & 0x7F, 0x24, "stepped down two first");
}
