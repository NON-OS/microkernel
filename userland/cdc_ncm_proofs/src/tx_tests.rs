// NONOS Operating System (AGPL-3.0-or-later)
//! One frame per NTB16 out, laid out and padded as cdc_ncm_fill_tx_frame
//! lays out and pads the first datagram of a block.

use nonos_usbnet::mock::Call;
use nonos_usbnet::{Bind, Nic};

use crate::device::{calls, Dev};
use crate::ncm::align::OutAlign;
use crate::ncm::bind;
use crate::ncm::ntb_out::build;
use crate::ncm::shape::TxShape;

#[test]
fn a_frame_goes_out_as_nth16_ndp16_and_the_datagram_on_its_divisor() {
    let dev = Dev::ncm(0);
    let bus = dev.bus();
    let Bind::Ours(mut nic) = bind(bus.clone(), &dev.found()) else { panic!("not bound") };
    nic.send(&[0x11; 60]).unwrap();
    let head: [u8; 30] = [
        0x4E, 0x43, 0x4D, 0x48, 0x0C, 0x00, 0x00, 0x00, 0x5A, 0x00, 0x0C, 0x00, // NTH16
        0x4E, 0x43, 0x4D, 0x30, 0x10, 0x00, 0x00, 0x00, 0x1E, 0x00, 0x3C, 0x00, // NDP16
        0x00, 0x00, 0x00, 0x00, // the zero entry
        0x00, 0x00, // so the payload after the Ethernet header lands on 44, divisor 4
    ];
    let want = [&head[..], &[0x11; 60]].concat();
    assert_eq!(calls(&bus).last(), Some(&Call::BulkOut(want)));
    nic.send(&[0x22; 60]).unwrap();
    let Some(Call::BulkOut(next)) = calls(&bus).last().cloned() else { panic!("no block") };
    assert_eq!(&next[6..8], &[1, 0], "wSequence counts blocks");
}

fn shape(max: usize, mps: usize, zlp: bool) -> TxShape {
    let min_pkt = crate::ncm::limits::min_tx_pkt(max, mps);
    TxShape { max, min_pkt, mps, zlp, align: OutAlign { ndp: 4, modulus: 4, remainder: 2 } }
}

#[test]
fn a_block_ending_on_a_packet_gets_one_byte_and_a_long_one_the_full_size() {
    let mut out = vec![0u8; 4095];
    let n = build(&mut out, &[1; 98], 0, &shape(4095, 512, false)).unwrap();
    assert_eq!(n, 129, "30 + 98 = 128, a multiple of 64");
    assert_eq!(&out[8..10], &129u16.to_le_bytes(), "wBlockLength is the padded length");
    assert_eq!(build(&mut out, &[1; 482], 0, &shape(4095, 512, false)), Some(513));
    let n = build(&mut out, &[1; 1514], 0, &shape(4095, 1024, false)).unwrap();
    assert_eq!(n, 4095, "past min_tx_pkt on SuperSpeed: padded to tx_max");
    assert!(out[1544..4095].iter().all(|&b| b == 0), "the padding is zeros");
    assert_eq!(build(&mut out, &[1; 1514], 0, &shape(4095, 1024, true)), Some(1544), "ZLP device");
    assert_eq!(build(&mut out, &[1; 1514], 0, &shape(4095, 512, false)), Some(1544));
}

#[test]
fn a_frame_that_does_not_fit_the_block_is_refused() {
    let mut out = vec![0u8; 4095];
    // cdc_ncm_ndp16 reserves the frame, the divisor and the remainder after
    // the NDP: 12 + 16 + 1514 + 4 + 2 bytes.
    assert_eq!(build(&mut out, &[1; 1514], 0, &shape(1547, 512, false)), None);
    assert_eq!(build(&mut out, &[1; 1514], 0, &shape(1548, 512, false)), Some(1548));
    assert_eq!(build(&mut out[..100], &[1; 60], 0, &shape(4095, 512, false)), None);
    let dev = Dev::ncm(0);
    let Bind::Ours(mut nic) = bind(dev.bus(), &dev.found()) else { panic!("not bound") };
    assert_eq!(nic.send(&[0; 4080]), Err(-22));
}
