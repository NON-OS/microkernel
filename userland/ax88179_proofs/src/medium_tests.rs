// NONOS Operating System (AGPL-3.0-or-later)
//! The medium mode and bulk IN row for each link ax88179_link_reset
//! knows, and the bulk IN rows fitted to driver.xhci0's BULK_MAX.

use nonos_usbnet::xhci::BULK_MAX;

use crate::ax::bulkin::{fitted, BULKIN_SIZE};
use crate::ax::medium::medium;

/// AX88179_BULKIN_SIZE, from ax88179_178a.c.
const LINUX_ROWS: [[u8; 5]; 4] = [
    [7, 0x4f, 0, 0x12, 0xff],
    [7, 0x20, 3, 0x16, 0xff],
    [7, 0xae, 7, 0x18, 0xff],
    [7, 0xcc, 0x4c, 0x18, 8],
];

fn with_size_2(row: usize) -> [u8; 5] {
    let mut r = LINUX_ROWS[row];
    r[3] = 2;
    r
}

#[test]
fn every_link_gets_linux_medium_and_row() {
    // PHYSR: link 0x400, full 0x2000, giga 0x8000, 100 0x4000.
    // Link status: SuperSpeed 0x04, high speed 0x02.
    let cases = [
        (0xa400, 0x04, 0x013b, 0),
        (0xa400, 0x02, 0x013b, 1),
        (0xa400, 0x00, 0x013b, 3),
        (0x8400, 0x04, 0x0139, 0),
        (0x6400, 0x04, 0x0332, 2),
        (0x4400, 0x02, 0x0330, 2),
        (0x6400, 0x00, 0x0332, 3),
        (0x2400, 0x04, 0x0132, 3),
        (0x0400, 0x02, 0x0130, 3),
    ];
    for (physr, sts, mode, row) in cases {
        let m = medium(physr, sts).unwrap();
        assert_eq!((m.mode, m.bulkin), (mode, with_size_2(row)), "{physr:#06x} {sts}");
    }
    assert_eq!(medium(0xa000, 0x04), None, "no link bit");
}

#[test]
fn linux_rows_overrun_bulk_max_and_fitted_rows_fill_it_exactly() {
    assert_eq!(BULKIN_SIZE, LINUX_ROWS);
    let urb = |r: [u8; 5]| 1024 * (r[3] as usize + 2);
    for row in LINUX_ROWS {
        assert!(urb(row) > BULK_MAX, "Linux's {} bytes", urb(row));
        let f = fitted(row);
        assert_eq!(urb(f), BULK_MAX);
        assert_eq!((f[0], f[1], f[2], f[4]), (row[0], row[1], row[2], row[4]), "timer kept");
    }
}
