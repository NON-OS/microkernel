// NONOS Operating System (AGPL-3.0-or-later)
//! The NTB parameters and the sizes derived from them, as Linux
//! cdc_ncm_check_rx_max, cdc_ncm_check_tx_max and cdc_ncm_update_rxtx_max
//! derive them.

use crate::ncm::limits::{min_tx_pkt, rx_max, tx_max};
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
fn the_output_size_never_ends_on_a_packet_unless_it_is_the_devices_own() {
    assert_eq!(tx_max(16384, 512), 4095, "BULK_MAX falls on a packet: one byte less");
    assert_eq!(tx_max(2048, 512), 2048, "dwNtbOutMaxSize itself needs no short packet");
    assert_eq!(tx_max(3000, 1024), 3000);
    assert_eq!(tx_max(0, 512), 1698, "none given: Linux's least, frame + 40-entry NDP + NTH");
    assert_eq!(tx_max(1000, 64), 1698, "below 2048 is a spec violation; Linux's least");
    assert_eq!(tx_max(2560, 512), 2560);
    assert_eq!(tx_max(3072, 1024), 3072);
}

#[test]
fn padding_to_the_full_block_starts_three_packets_short_of_it() {
    assert_eq!(min_tx_pkt(4095, 512), 4095 - 1536);
    assert_eq!(min_tx_pkt(4095, 1024), 1023);
    assert_eq!(min_tx_pkt(1698, 512), 512, "CDC_NCM_MIN_TX_PKT");
    assert_eq!(min_tx_pkt(300, 512), 300, "never past the block");
}
