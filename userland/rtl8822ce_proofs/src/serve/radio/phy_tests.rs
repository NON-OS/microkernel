// NONOS Operating System (AGPL-3.0-or-later)
//! The serve loop's PHY entry over an inert register page: a front end
//! rtw88 has no table for, or no efuse at all, is refused before any
//! register is written; a supported board comes up with a drawn, locally
//! administered station address, both paths on channel 1, and its power.

use super::phy::phy_setup;
use crate::phy::bits::read32_mask;
use crate::phy::rf::set_paths;
use crate::regs::{Mmio, Regs};
use crate::serve::Stage;

const BAR: usize = 0x1_0000;

fn page() -> (Regs, &'static [u64]) {
    let mem: &'static mut [u64] = Box::leak(vec![0u64; BAR / 8].into_boxed_slice());
    // SAFETY: `mem` is BAR zeroed bytes, leaked, so they outlive the window.
    let regs = unsafe { Regs::new(mem.as_ptr() as u64, BAR as u64) };
    (regs, mem)
}

fn board(rfe: u8) -> crate::efuse::EfuseInfo {
    crate::tests::fixtures::board(3, rfe)
}

#[test]
fn an_unknown_front_end_is_refused_untouched() {
    let (regs, mem) = page();
    for rfe in [7, 0x15, 0xFF] {
        assert!(matches!(phy_setup(&regs, Some(board(rfe))), Err(Stage::EfuseFailed)));
    }
    assert!(mem.iter().all(|&w| w == 0), "no register written");
}

#[test]
fn no_efuse_is_refused() {
    let (regs, _) = page();
    assert!(matches!(phy_setup(&regs, None), Err(Stage::EfuseFailed)));
}

#[test]
fn a_supported_board_comes_up() {
    let _one = crate::tests::lock();
    set_paths(2);
    let (regs, _) = page();
    assert!(phy_setup(&regs, Some(board(0))).is_ok());
    let mac = [regs.read32(0x0610).to_le_bytes(), regs.read32(0x0614).to_le_bytes()].concat();
    assert_eq!(mac[0] & 0x03, 0x02, "locally administered, unicast");
    assert!(mac[..6].iter().any(|&b| b != 0));
    for rf18 in [0x3C00 + (0x18 << 2), 0x4C00 + (0x18 << 2)] {
        assert_eq!(read32_mask(&regs, rf18, 0xFF), 1, "channel 1");
    }
    assert_eq!(read32_mask(&regs, 0x18A0, 0x7F_0000), 0x3C, "path A's CCK power");
    assert_eq!(regs.read16(0x00AA), 0x8003, "the scoreboard says Wi-Fi is on");
}
