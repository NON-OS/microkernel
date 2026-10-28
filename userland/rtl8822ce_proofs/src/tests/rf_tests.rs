// NONOS Operating System (AGPL-3.0-or-later)
//! The 8822C's RF access: reads straight from each path's window (0x3C00,
//! 0x4C00), writes there too for every register but 0, which goes through
//! the path's serial interface (0x1808, 0x4108); a path the chip lacks is
//! neither read nor written.

use crate::model::Chip;
use crate::phy::rf::{paths, read, set_paths, write, INV_RF_DATA, PATH_A, PATH_B, RFREG_MASK};

#[test]
fn reads_come_from_each_paths_window() {
    let _one = super::lock();
    set_paths(2);
    let c = Chip::new();
    c.set32(0x3C00 + (0x18 << 2), 0xFFF1_2345);
    c.set32(0x4C00 + (0x18 << 2), 0x0006_789A);
    assert_eq!(read(&c, PATH_A, 0x18, RFREG_MASK), 0x1_2345);
    assert_eq!(read(&c, PATH_B, 0x18, RFREG_MASK), 0x6_789A);
    assert_eq!(read(&c, PATH_A, 0x18, 0xFF), 0x45);
}

#[test]
fn a_register_but_0_is_written_in_place() {
    let _one = super::lock();
    set_paths(2);
    let c = Chip::new();
    c.set32(0x4C00 + (0xEE << 2), 0xABC0_0000);
    assert!(write(&c, PATH_B, 0xEE, 1 << 19, 1));
    assert_eq!(c.at(0x4C00 + (0xEE << 2)), [0xABC8_0000]);
    assert!(c.at(0x4108).is_empty(), "not through the serial interface");
}

#[test]
fn register_0_goes_through_the_serial_interface() {
    let _one = super::lock();
    set_paths(2);
    let c = Chip::new();
    assert!(write(&c, PATH_A, 0x0, RFREG_MASK, 0x1_0000));
    assert_eq!(c.at(0x1808), [0x1_0000]);
    assert!(write(&c, PATH_A, 0x0, RFREG_MASK, 0xFFF0_0001));
    assert_eq!(c.at(0x1808)[1], 0x1, "the 20 data bits alone");
    c.set32(0x4C00, 0x0003_0000);
    assert!(write(&c, PATH_B, 0x0, 0xF, 0x5));
    assert_eq!(c.at(0x4108), [0x3_0005], "a masked write keeps the rest");
}

#[test]
fn a_path_the_chip_lacks_is_left_alone() {
    let _one = super::lock();
    set_paths(1);
    let c = Chip::new();
    assert!(!write(&c, PATH_B, 0x18, RFREG_MASK, 1));
    assert_eq!(read(&c, PATH_B, 0x18, RFREG_MASK), INV_RF_DATA);
    assert!(c.writes.borrow().is_empty());
    set_paths(0);
    assert_eq!(paths(), 1, "never fewer than one");
    set_paths(9);
    assert_eq!(paths(), 2, "never more than two");
}
