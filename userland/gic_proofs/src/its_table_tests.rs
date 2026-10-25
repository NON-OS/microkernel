// NONOS Operating System (AGPL-3.0-or-later)
//! The ITS's table registers and the redistributor's LPI tables.

use crate::gic::its::encode::attrs::{cbaser, coherent};
use crate::gic::its::encode::baser::{
    entry_bytes, page_bytes, page_of, table, table_type, PAGE_16K, PAGE_4K, PAGE_64K,
    TYPE_COLLECTIONS, TYPE_DEVICES,
};
use crate::gic::its::encode::lpi::{config, pend_bytes, pendbaser, prop_bytes, propbaser};

const V: u64 = 1 << 63;
const WB_INNER: u64 = 7 << 59 | 1 << 10;

#[test]
fn a_table_keeps_its_type_and_entry_size_and_takes_the_rest() {
    let read = 1 << 56 | 7 << 48 | 0x1234_5678;
    assert_eq!((table_type(read), entry_bytes(read)), (TYPE_DEVICES, 8));
    assert_eq!(table_type(4 << 56), TYPE_COLLECTIONS);
    let at = 0x4100_0000;
    assert_eq!(table(read, at, PAGE_4K, 128, true), V | 1 << 56 | 7 << 48 | WB_INNER | at | 127);
    let plain = table(read, at, PAGE_16K, 1, false);
    assert_eq!(plain, V | 1 << 56 | 7 << 48 | 1 << 59 | at | PAGE_16K);
    let high = 0x0003_0004_0001_0000;
    assert_eq!(
        table(0, high, PAGE_64K, 256, true),
        V | WB_INNER | 0x4_0001_0000 | 3 << 12 | 2 << 8 | 255
    );
    assert_eq!([PAGE_4K, PAGE_16K, PAGE_64K].map(page_bytes), [0x1000, 0x4000, 0x1_0000]);
    assert_eq!(page_of(table(0, at, PAGE_64K, 1, true)), PAGE_64K);
}

#[test]
fn shareability_read_back_decides_coherency() {
    assert!(coherent(1 << 10) && coherent(2 << 10) && !coherent(0) && !coherent(1 << 59));
    assert_eq!(cbaser(0x4000_0000, 1, true), V | WB_INNER | 0x4000_0000);
    assert_eq!(cbaser(0x4000_0fff, 16, false), V | 1 << 59 | 0x4000_0000 | 15);
}

#[test]
fn lpi_tables_and_configuration_bytes() {
    assert_eq!((config(true), config(false)), (0xa3, 0xa2));
    assert_eq!(propbaser(0x4000_0000, 14, true), 0x4000_0000 | 7 << 7 | 1 << 10 | 13);
    assert_eq!(propbaser(0x4000_0000, 14, false), 0x4000_0000 | 1 << 7 | 13);
    assert_eq!(pendbaser(0x4001_0000, false), 1 << 62 | 0x4001_0000 | 1 << 7);
    assert_eq!(pendbaser(0x4001_8000, true), 1 << 62 | 0x4001_0000 | 7 << 7 | 1 << 10);
    assert_eq!((prop_bytes(14), pend_bytes(14)), (8192, 2048));
    assert_eq!((prop_bytes(16), pend_bytes(16)), (57344, 8192));
}
