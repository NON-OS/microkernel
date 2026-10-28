// NONOS Operating System (AGPL-3.0-or-later)
//! The engines get the 8822C's FIFO: 2048 pages of 128 bytes, 110 reserved
//! (16 driver, 24 + 8 H2C info, 8 H2C queue, 4 firmware TX, 50 CSI), 64 to
//! each priority queue and the rest public; the receive FIFO bounded at
//! 24 KiB less the C2H buffer; the H2C queue right below the firmware's.

use crate::mac::{init_trx_cfg, RSVD_BOUNDARY};
use crate::model::Chip;

#[test]
fn the_reserved_boundary_is_the_8822cs() {
    assert_eq!(RSVD_BOUNDARY, 2048 - 110);
}

#[test]
fn the_engines_get_the_8822cs_pages() {
    let c = Chip::new();
    assert!(init_trx_cfg(&c));
    assert_eq!(c.at(0x0230), [64]); // FIFOPAGE_INFO_1: high
    assert_eq!(c.at(0x0234), [64]); // _2: low
    assert_eq!(c.at(0x0238), [64]); // _3: normal
    assert_eq!(c.at(0x023C), [64]); // _4: extra
    assert_eq!(c.at(0x0240), [1938 - 4 * 64 - 1]); // _5: public, less the gap page
    assert_eq!(c.at(0x0204), [1938]); // FIFOPAGE_CTRL_2
    assert_eq!(c.at(0x0424), [1938]); // BCNQ_BDNY_V1
    assert_eq!(c.at(0x011C), [24576 - 256 - 1]); // RXFF_BNDY
}

#[test]
fn the_h2c_queue_sits_below_the_firmware_pages() {
    let c = Chip::new();
    assert!(init_trx_cfg(&c));
    // 2048 - 50 CSI - 4 FW TX - 0 CPU - 8 H2C = page 1986.
    assert_eq!(c.get32(0x0244) & 0x3_FFFF, 1986 << 7);
    assert_eq!(c.get32(0x024C) & 0x3_FFFF, 1986 << 7);
    assert_eq!(c.get32(0x0248) & 0x3_FFFF, (1986 + 8) << 7);
}

#[test]
fn the_queue_mapping_is_rtw88s_pcie_row() {
    let c = Chip::new();
    assert!(init_trx_cfg(&c));
    // VO, VI normal (2); BE, BK low (1); MG extra (0); HI high (3).
    assert_eq!(c.at(0x010C), [2 << 4 | 2 << 6 | 1 << 8 | 1 << 10 | 3 << 14]);
}
