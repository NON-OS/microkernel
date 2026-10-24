// NONOS Operating System (AGPL-3.0-or-later)
//! One frame per NTB16 out (NCM 1.0, 3.2.1 and 3.3.1, as recalled,
//! unverified): laid out on the device's alignment, and one byte longer
//! only when it would end on a packet boundary below dwNtbOutMaxSize.

use nonos_usbnet::mock::{drawn, Call};
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
    let Bind::Ours(mut nic) = bind(bus.clone(), &dev.found(), drawn()) else { panic!("not bound") };
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

fn shape(max: usize, out_max: usize, mps: usize) -> TxShape {
    TxShape { max, out_max, mps, align: OutAlign { ndp: 4, modulus: 4, remainder: 2 } }
}

#[test]
fn a_block_ending_on_a_packet_below_the_devices_size_gets_one_byte() {
    let mut out = vec![0u8; 4096];
    let n = build(&mut out, &[1; 98], 0, &shape(4095, 16384, 512)).unwrap();
    assert_eq!(n, 129, "30 + 98 = 128, a multiple of 64");
    assert_eq!(&out[8..10], &129u16.to_le_bytes(), "wBlockLength is the padded length");
    assert_eq!(out[128], 0, "the byte is zero");
    assert_eq!(build(&mut out, &[1; 482], 0, &shape(4095, 16384, 512)), Some(513));
    assert_eq!(build(&mut out, &[1; 1514], 0, &shape(4095, 16384, 1024)), Some(1544));
    assert_eq!(build(&mut out, &[1; 98], 0, &shape(128, 128, 64)), Some(128), "its own size");
    assert_eq!(build(&mut out, &[1; 4066], 0, &shape(4095, 16384, 512)), None, "past 4095");
    assert_eq!(build(&mut out, &[1; 4002], 0, &shape(4095, 16384, 512)), Some(4033));
}

#[test]
fn a_frame_that_does_not_fit_the_block_is_refused() {
    let mut out = vec![0u8; 4096];
    // 12 + 16 + 2 bytes to reach the remainder, then the frame.
    assert_eq!(build(&mut out, &[1; 1514], 0, &shape(1543, 1543, 512)), None);
    assert_eq!(build(&mut out, &[1; 1514], 0, &shape(1544, 1544, 512)), Some(1544));
    assert_eq!(build(&mut out[..100], &[1; 60], 0, &shape(4095, 16384, 512)), Some(90));
    assert_eq!(build(&mut out[..89], &[1; 60], 0, &shape(4095, 16384, 512)), None, "out short");
    let dev = Dev::ncm(0);
    let Bind::Ours(mut nic) = bind(dev.bus(), &dev.found(), drawn()) else { panic!("not bound") };
    assert_eq!(nic.send(&[0; 4080]), Err(-22));
}
