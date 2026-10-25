// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! The rings, each side against the other.

use std::vec;
use std::vec::Vec;

use crate::mem::{Buffer, Shared};
use crate::model::HostRing;
use crate::packet::{gpa_direct, Header, FLAG_REQUEST_COMPLETION, PACKET_DATA_INBAND};
use crate::ring::{Ring, RingError, PAGE};

/// Two rings of `pages` each: the guest's at page 0, the host's after.
fn rings(pages: usize) -> (Buffer, Ring, Ring, HostRing, HostRing) {
    let mut m = Buffer(vec![0u8; 2 * pages * PAGE]);
    let out = Ring::new(0, pages);
    let inb = Ring::new(pages * PAGE, pages);
    out.init(&mut m);
    inb.init(&mut m);
    let len = ((pages - 1) * PAGE) as u32;
    (
        m,
        out,
        inb,
        HostRing { hdr: 0, len },
        HostRing {
            hdr: pages * PAGE,
            len,
        },
    )
}

fn inband(txid: u64) -> Header {
    Header::new(PACKET_DATA_INBAND, FLAG_REQUEST_COMPLETION, txid)
}

#[test]
fn a_fresh_ring_is_empty_unmasked_and_honours_pending() {
    let (m, out, _, _, _) = rings(2);
    assert_eq!(out.pending(&m), Ok(false));
    assert_eq!(m.load32(64), 1, "pending_send_sz honoured");
    assert_eq!(
        (m.load32(0), m.load32(4), m.load32(8), m.load32(12)),
        (0, 0, 0, 0)
    );
}

#[test]
fn a_guest_packet_reads_back_as_the_host_reads_it() {
    let (mut m, out, _, hout, _) = rings(2);
    let data = [7u8; 13];
    assert_eq!(
        out.send(&mut m, &inband(42), &[], &[&data[..5], &data[5..]]),
        Ok(true)
    );
    let (ty, flags, txid, desc, got) = hout.recv(&mut m).unwrap();
    assert_eq!(
        (ty, flags, txid),
        (PACKET_DATA_INBAND, FLAG_REQUEST_COMPLETION, 42)
    );
    assert!(desc.is_empty());
    assert_eq!(&got[..13], &data[..], "padded to 8 with zeroes after");
    assert_eq!(&got[13..], &[0, 0, 0]);
    assert!(hout.recv(&mut m).is_none());
}

#[test]
fn a_descriptor_comes_before_the_data() {
    let (mut m, out, _, hout, _) = rings(2);
    let mut desc = [0u8; 32];
    let n = gpa_direct(&mut desc, 6000, 100, &[0x1234, 0x1235]).unwrap();
    assert_eq!(n, 32);
    out.send(&mut m, &Header::new(9, 1, 5), &desc[..n], &[b"cmd"])
        .unwrap();
    let (ty, _, _, d, data) = hout.recv(&mut m).unwrap();
    assert_eq!(ty, 9);
    assert_eq!(d, desc.to_vec());
    assert_eq!(&data[..3], b"cmd");
    assert_eq!(
        u32::from_le_bytes(d[4..8].try_into().unwrap()),
        1,
        "one range"
    );
    assert_eq!(u32::from_le_bytes(d[8..12].try_into().unwrap()), 6000);
    assert_eq!(u32::from_le_bytes(d[12..16].try_into().unwrap()), 100);
    assert_eq!(gpa_direct(&mut [0u8; 31], 1, 0, &[1, 2]), None);
}

#[test]
fn a_host_packet_reads_back_as_the_guest_reads_it() {
    let (mut m, _, inb, _, hin) = rings(2);
    assert_eq!(hin.send(&mut m, 11, 9, b"answer!!x"), Some(true));
    let mut buf = [0u8; 64];
    let (h, n, signal) = inb.recv(&mut m, &mut buf).unwrap().unwrap();
    assert_eq!(
        (h.ty, h.transaction_id, h.data_at, n, signal),
        (11, 9, 16, 32, false)
    );
    assert_eq!(&buf[16..25], b"answer!!x");
    assert_eq!(inb.recv(&mut m, &mut buf), Ok(None));
}

#[test]
fn both_sides_wrap_around_the_ring_many_times() {
    let (mut m, out, inb, hout, hin) = rings(2);
    let mut buf = [0u8; 4096];
    for i in 0..2000u64 {
        let size = (i as usize * 37) % 900 + 1;
        let data: Vec<u8> = (0..size).map(|b| (b as u64 + i) as u8).collect();
        out.send(&mut m, &inband(i), &[], &[&data]).unwrap();
        let (_, _, txid, _, got) = hout.recv(&mut m).unwrap();
        assert_eq!((txid, &got[..size]), (i, &data[..]), "guest packet {i}");
        assert!(hin.send(&mut m, 6, i, &data).is_some(), "host packet {i}");
        let (h, n, _) = inb.recv(&mut m, &mut buf).unwrap().unwrap();
        assert_eq!(
            (h.transaction_id, &buf[h.data_at..h.data_at + size]),
            (i, &data[..])
        );
        assert!(n >= 16 + size);
    }
}

#[test]
fn the_guest_never_fills_the_ring() {
    let (mut m, out, _, hout, _) = rings(2);
    let len = (PAGE - 1) as u32;
    // 16 header + data + 8 trailer: fill to exactly one byte short.
    let big = vec![0u8; 4096 - 16 - 8 - 8];
    assert_eq!(out.send(&mut m, &inband(1), &[], &[&big]), Ok(true));
    let used = m.load32(0);
    assert_eq!(used, 4088);
    assert_eq!(
        out.send(&mut m, &inband(2), &[], &[&[0u8; 1]]),
        Err(RingError::Full)
    );
    hout.recv(&mut m).unwrap();
    // Now 4095 bytes free: a packet of 4088 fits, one of 4096 never does.
    assert_eq!(out.send(&mut m, &inband(3), &[], &[&big]), Ok(true));
    hout.recv(&mut m).unwrap();
    let too_big = vec![0u8; 4096 - 16 - 8];
    assert_eq!(
        out.send(&mut m, &inband(4), &[], &[&too_big]),
        Err(RingError::Full)
    );
    let _ = len;
}

#[test]
fn the_guest_signals_only_a_host_that_may_be_waiting() {
    let (mut m, out, _, hout, _) = rings(4);
    assert_eq!(
        out.send(&mut m, &inband(1), &[], &[b"a"]),
        Ok(true),
        "host caught up"
    );
    assert_eq!(
        out.send(&mut m, &inband(2), &[], &[b"b"]),
        Ok(false),
        "host still behind"
    );
    hout.recv(&mut m).unwrap();
    hout.recv(&mut m).unwrap();
    m.store32(8, 1);
    assert_eq!(
        out.send(&mut m, &inband(3), &[], &[b"c"]),
        Ok(false),
        "the host masked us"
    );
}

#[test]
fn the_guest_signals_a_host_waiting_for_room_once_it_has_it() {
    let (mut m, _, inb, _, hin) = rings(2);
    let mut buf = [0u8; 4096];
    let chunk = vec![1u8; 1000];
    let mut sent = 0;
    while hin.send(&mut m, 6, sent, &chunk).is_some() {
        sent += 1;
    }
    assert_eq!(sent, 3);
    assert!(
        m.load32(PAGE * 2 + 12) > 0,
        "the host said what it waits for"
    );
    let (_, _, signal) = inb.recv(&mut m, &mut buf).unwrap().unwrap();
    assert!(signal, "room enough now");
    // Nothing waiting: no signal.
    let (_, _, signal) = inb.recv(&mut m, &mut buf).unwrap().unwrap();
    assert!(!signal);
}

#[test]
fn an_index_past_the_ring_breaks_it() {
    let (mut m, out, inb, _, _) = rings(2);
    let mut buf = [0u8; 64];
    for (at, v) in [(0, 4096), (4, 4096), (0, u32::MAX)] {
        let (mut m2, _, _, _, _) = rings(2);
        m2.store32(at, v);
        assert_eq!(
            out.send(&mut m2, &inband(1), &[], &[b"x"]),
            Err(RingError::Broken)
        );
        assert_eq!(out.pending(&m2), Err(RingError::Broken));
    }
    // The last byte of the data is an index like any other.
    let (mut m3, out3, _, _, _) = rings(2);
    m3.store32(0, 4095);
    m3.store32(4, 4095);
    assert_eq!(out3.send(&mut m3, &inband(1), &[], &[b"x"]), Ok(true));
    m.store32(2 * PAGE, 9000);
    assert_eq!(inb.recv(&mut m, &mut buf), Err(RingError::Broken));
}

#[test]
fn a_malformed_host_packet_is_refused_and_left_unread() {
    let mut buf = [0u8; 4096];
    // offset under the header, offset past the end, end past what was written.
    for (offset, len, written) in [(1u16, 4u16, 40u32), (5, 4, 40), (2, 10, 40), (2, 2, 8)] {
        let (mut m, _, inb, _, _) = rings(2);
        let mut hdr = [0u8; 16];
        hdr[2..4].copy_from_slice(&offset.to_le_bytes());
        hdr[4..6].copy_from_slice(&len.to_le_bytes());
        m.write(3 * PAGE, &hdr);
        m.store32(2 * PAGE, written);
        assert_eq!(
            inb.recv(&mut m, &mut buf),
            Err(RingError::Malformed),
            "{offset} {len} {written}"
        );
        assert_eq!(m.load32(2 * PAGE + 4), 0, "the read index stays");
    }
}

#[test]
fn a_packet_too_big_for_the_buffer_is_refused_then_skipped() {
    let (mut m, _, inb, _, hin) = rings(2);
    hin.send(&mut m, 6, 1, &[3u8; 200]).unwrap();
    hin.send(&mut m, 6, 2, &[4u8; 8]).unwrap();
    let mut small = [0u8; 64];
    assert_eq!(inb.recv(&mut m, &mut small), Err(RingError::TooBig));
    inb.skip(&mut m).unwrap();
    let (h, _, _) = inb.recv(&mut m, &mut small).unwrap().unwrap();
    assert_eq!(h.transaction_id, 2);
}

#[test]
fn a_header_that_cannot_encode_is_refused() {
    let (mut m, out, _, _, _) = rings(2);
    assert_eq!(
        out.send(&mut m, &inband(1), &[0u8; 7], &[]),
        Err(RingError::TooBig),
        "descriptor not 8-aligned"
    );
    assert!(
        Header::new(6, 0, 0).sized(0, 8 * 70_000).is_none(),
        "past u16 qwords"
    );
}

#[test]
fn a_host_packet_with_no_data_parses_and_one_ending_early_does_not() {
    let mut b = [0u8; 16];
    b[0] = 0xb;
    b[2] = 2;
    b[4] = 2;
    let h = Header::parse(&b).unwrap();
    assert_eq!(
        (h.ty, h.data_at, h.end),
        (0xb, 16, 16),
        "a completion with no payload"
    );
    b[2] = 3;
    assert_eq!(Header::parse(&b), None, "its data would start past its end");
}

#[test]
fn a_ring_holds_all_but_its_header_page() {
    assert_eq!(Ring::new(0, 2).data_len(), 4096);
    assert_eq!(Ring::new(0, 33).data_len(), 32 * 4096);
}

#[test]
fn a_ring_after_the_first_keeps_its_own_indexes() {
    let (mut m, _, inb, _, hin) = rings(2);
    // The guest writing on the second ring, as on a channel whose rings
    // were laid out the other way.
    assert_eq!(inb.send(&mut m, &inband(9), &[], &[b"x"]), Ok(true));
    assert_eq!(m.load32(0), 0, "the first ring untouched");
    assert_eq!(m.load32(2 * PAGE), 32);
    assert_eq!(hin.recv(&mut m).unwrap().2, 9);
}

#[test]
fn a_descriptor_and_odd_data_pad_to_eight() {
    let (mut m, out, _, hout, _) = rings(2);
    out.send(&mut m, &inband(1), &[1u8; 8], &[b"abc"]).unwrap();
    assert_eq!(
        m.load32(0),
        16 + 8 + 8 + 8,
        "header, descriptor, data padded, trailer"
    );
    let (_, _, _, desc, data) = hout.recv(&mut m).unwrap();
    assert_eq!((desc, data), (vec![1u8; 8], b"abc\0\0\0\0\0".to_vec()));
}

#[test]
fn a_header_only_packet_is_taken_into_a_buffer_its_size() {
    let (mut m, _, inb, _, hin) = rings(2);
    hin.send(&mut m, 0xb, 3, &[]).unwrap();
    hin.send(&mut m, 0xb, 4, &[0u8; 8]).unwrap();
    let mut buf = [0u8; 16];
    let (h, n, _) = inb.recv(&mut m, &mut buf).unwrap().unwrap();
    assert_eq!((h.transaction_id, n), (3, 16));
    let mut buf = [0u8; 24];
    let (h, n, _) = inb.recv(&mut m, &mut buf).unwrap().unwrap();
    assert_eq!((h.transaction_id, n), (4, 24));
    assert_eq!(inb.recv(&mut m, &mut buf), Ok(None));
}

#[test]
fn a_packet_whose_trailer_is_not_yet_written_is_refused() {
    let (mut m, _, inb, _, hin) = rings(2);
    hin.send(&mut m, 6, 1, &[5u8; 8]).unwrap();
    m.store32(2 * PAGE, 24);
    let mut buf = [0u8; 64];
    assert_eq!(inb.recv(&mut m, &mut buf), Err(RingError::Malformed));
    assert_eq!(inb.skip(&mut m), Err(RingError::Malformed));
    assert_eq!(m.load32(2 * PAGE + 4), 0, "nothing taken");
    m.store32(2 * PAGE, 16);
    assert_eq!(
        inb.recv(&mut m, &mut buf),
        Err(RingError::Malformed),
        "not even a header"
    );
    m.store32(2 * PAGE, 32);
    inb.skip(&mut m).unwrap();
    assert_eq!(m.load32(2 * PAGE + 4), 32);
}

#[test]
fn a_waiting_host_is_signalled_exactly_when_its_room_is_there() {
    // Three packets of 32 bytes unread: 3999 free. Taking one frees 4031.
    for (wanted, signal) in [
        (4031, true),
        (4032, false),
        (4000, true),
        (3999, false),
        (0, false),
    ] {
        let (mut m, _, inb, _, hin) = rings(2);
        for t in 0..3 {
            hin.send(&mut m, 6, t, &[0u8; 8]).unwrap();
        }
        m.store32(2 * PAGE + 12, wanted);
        let mut buf = [0u8; 64];
        let (_, _, s) = inb.recv(&mut m, &mut buf).unwrap().unwrap();
        assert_eq!(s, signal, "the host waiting for {wanted}");
    }
}

#[test]
fn a_transfer_pages_descriptor_reads_as_its_set_and_ranges() {
    use crate::packet::xfer_pages;
    let mut d = vec![0xEF, 0xBE, 0xAA, 0x55, 2, 0, 0, 0];
    d.extend_from_slice(&[60, 0, 0, 0, 0, 0, 0, 0, 0xEA, 5, 0, 0, 0x0E, 7, 0, 0]);
    let (set, ranges) = xfer_pages(&d).unwrap();
    assert_eq!(set, 0xBEEF, "the reserved half may hold anything");
    assert_eq!(ranges.collect::<Vec<_>>(), [(60, 0), (0x5EA, 0x70E)]);
    assert!(xfer_pages(&d[..16]).is_none(), "fewer ranges than the count");
    assert!(xfer_pages(&d[..7]).is_none());
    let mut huge = d.clone();
    huge[4..8].copy_from_slice(&u32::MAX.to_le_bytes());
    assert!(xfer_pages(&huge).is_none());
}

#[test]
fn a_host_packet_with_a_descriptor_reads_back_with_its_data_after_it() {
    let (mut m, _, inb, _, hin) = rings(2);
    let desc = [1u8, 0, 0, 0, 1, 0, 0, 0, 9, 0, 0, 0, 4, 0, 0, 0];
    hin.send_packet(&mut m, 7, 1, 33, &desc, b"nvsp").unwrap();
    let mut buf = [0u8; 128];
    let (h, n, _) = inb.recv(&mut m, &mut buf).unwrap().unwrap();
    assert_eq!((h.ty, h.flags, h.transaction_id, h.data_at), (7, 1, 33, 32));
    assert_eq!(&buf[16..32], &desc);
    assert_eq!(&buf[32..36], b"nvsp");
    assert_eq!(n, 40);
}
