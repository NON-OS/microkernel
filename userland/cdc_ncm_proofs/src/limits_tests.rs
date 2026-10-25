// NONOS Operating System (AGPL-3.0-or-later)
//! The NTB parameters and the sizes derived from them (NCM 1.0, 6.2.1 and
//! 6.2.7, as recalled, unverified), held to one bulk transfer.

use crate::ncm::limits::{rx_max, tx_max};
use crate::ncm::params::parse_params;
use crate::spec::ntb_params;

#[test]
fn table_6_3_is_read_field_by_field() {
    let p = parse_params(&ntb_params(3, 0x1234_5678, 0x0000_9ABC, (8, 2, 16))).unwrap();
    assert_eq!((p.formats, p.in_max, p.out_max), (3, 0x1234_5678, 0x9ABC));
    assert_eq!((p.out_divisor, p.out_remainder, p.out_alignment), (8, 2, 16));
    assert!(p.ntb32());
    assert_eq!(parse_params(&ntb_params(1, 0, 0, (4, 0, 4))[..27]), None);
}

#[test]
fn the_input_size_is_one_transfer_at_most_and_2048_at_least() {
    let cases = [(65536, 4096), (16384, 4096), (4096, 4096), (3000, 3000), (2048, 2048)];
    for (offered, asked) in cases.into_iter().chain([(1024, 2048), (0, 2048), (u32::MAX, 4096)]) {
        assert_eq!(rx_max(offered), asked, "{offered}");
    }
}

#[test]
fn the_output_size_is_the_devices_own_or_one_byte_under_a_transfer() {
    assert_eq!(tx_max(16384), 4095, "past one transfer: room for the byte that ends it");
    assert_eq!(tx_max(4097), 4095);
    assert_eq!(tx_max(4096), 4096, "dwNtbOutMaxSize itself needs no byte");
    assert_eq!((tx_max(2048), tx_max(3000), tx_max(2560)), (2048, 3000, 2560));
    assert_eq!((tx_max(1000), tx_max(0)), (1000, 0), "too small: tx_shape refuses it");
}
