// NONOS Operating System (AGPL-3.0-or-later)
//! The receive ring under hostile input: every buffer that is not one
//! whole, good 802.11 frame is dropped and counted, and the next is read.

use super::ring_tests::{data_to_ap, tuned, AP};
use crate::model::rxq::{wifi, wrap};
use nonos_wifi_core::softmac::Radio;

#[test]
fn hostile_receive_buffers_are_dropped_and_the_next_frame_read() {
    let (card, mut sta) = tuned();
    let mut out = [0u8; 2304];
    // The firmware's command acks (C2H packets) are buffers too: not frames.
    assert!(sta.recv(&mut out).is_none());
    let before = sta.counts;
    let good = data_to_ap(AP);
    let mut bad_crc = wifi(&good);
    bad_crc[4 + 13] |= 0x02; // w3 bit 9: CRC32 error
    let mut split = wifi(&good);
    split[1] &= !0x40; // first segment only
    {
        let mut s = card.0.borrow_mut();
        s.rxq.push_back(wrap(1, &[0; 40])); // a PPDU status, not a frame
        s.rxq.push_back(bad_crc);
        s.rxq.push_back(split);
        s.rxq.push_back(wrap(0, &[0x08, 0, 0, 0])); // shorter than a header
        s.rxq.push_back(wifi(&vec![0x08; 3000])); // longer than the caller's room
        s.rxq.push_back(wifi(&good));
    }
    let got = sta.recv(&mut out).expect("the good frame");
    assert_eq!(&out[..got.len], &good[..]);
    let c = sta.counts;
    assert_eq!((c.rx_drop - before.rx_drop, c.rx_bad_crc - before.rx_bad_crc), (4, 1));
    assert!(sta.recv(&mut out).is_none());
    assert_eq!(card.0.borrow().violations, Vec::<String>::new());
}
