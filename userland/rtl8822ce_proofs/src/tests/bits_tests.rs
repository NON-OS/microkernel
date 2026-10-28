// NONOS Operating System (AGPL-3.0-or-later)
//! rtw88's field helpers: a masked write keeps what is outside the mask and
//! shifts the value to the mask, reading first even for a whole-word mask; a
//! masked read shifts the field down; the ready check gives up in time.

use crate::model::Chip;
use crate::phy::bits::{read32_mask, ready, write32_mask, write8_mask};
use crate::regs::Mmio;

#[test]
fn a_masked_write_keeps_the_rest() {
    let c = Chip::new();
    c.set32(0x1C80, 0xA5A5_A5A5);
    write32_mask(&c, 0x1C80, 0x3F00_0000, 0xF);
    assert_eq!(c.read32(0x1C80), 0x8FA5_A5A5);
    write32_mask(&c, 0x1C80, 0x7F, 0x1FF);
    assert_eq!(c.read32(0x1C80), 0x8FA5_A5FF, "a value wider than its mask is cut");
}

#[test]
fn a_masked_read_shifts_down() {
    let c = Chip::new();
    c.set32(0x1D70, 0x0000_3A2C);
    assert_eq!(read32_mask(&c, 0x1D70, 0x7F00), 0x3A);
    assert_eq!(read32_mask(&c, 0x1D70, 0x7F), 0x2C);
}

#[test]
fn a_byte_mask_write() {
    let c = Chip::new();
    c.mem.borrow_mut().insert(0x0790, 0xC0);
    write8_mask(&c, 0x0790, 0x3F, 0x5);
    assert_eq!(c.read8(0x0790), 0xC5);
}

#[test]
fn ready_reads_the_field_and_gives_up() {
    let c = Chip::new();
    c.set32(0x2808, 0x007F_FF80);
    assert!(ready(&c, 0x2808, 0x7F_FF80, 0xFFFF));
    c.set32(0x2808, 0x007F_FF00);
    assert!(!ready(&c, 0x2808, 0x7F_FF80, 0xFFFF));
}
