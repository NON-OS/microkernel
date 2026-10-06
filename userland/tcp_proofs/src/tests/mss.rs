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

//! The segment size a peer announces in its SYN is the largest it is sent,
//! and the option area it comes in is read without stepping out of it.

use crate::peer::{connect, encode, fresh, send, sent, set_auto, Auto, Seg, ACK, LOCAL, REMOTE, SYN};

/// The payload sizes of the data segments one 1460 byte write goes out as,
/// to a peer whose SYN-ACK announced `mss`.
fn sizes(mss: Option<u16>) -> Vec<usize> {
    set_auto(Auto { mss, ..Auto::plain() });
    let h = connect();
    let _ = sent();
    assert_eq!(send(h, &[7u8; 1460]), 0);
    sent().iter().filter(|s| !s.payload.is_empty()).map(|s| s.payload.len()).collect()
}

/*
 * Segments go out with Don't Fragment set. One larger than the path takes is
 * dropped at the narrow link (PPPoE, a tunnel) and the ICMP that says so is
 * not read, so a peer behind one stalled on every full segment.
 */
#[test]
fn segments_never_exceed_the_mss_the_peer_announced() {
    let _g = fresh();
    assert_eq!(sizes(Some(536)), vec![536, 536, 388]);
}

#[test]
fn a_peer_that_announces_none_is_sent_536() {
    let _g = fresh();
    assert_eq!(sizes(None), vec![536, 536, 388], "RFC 9293 3.7.1");
}

#[test]
fn an_announced_mss_above_ours_is_held_to_ours() {
    let _g = fresh();
    assert_eq!(sizes(Some(9000)), vec![1460]);
}

#[test]
fn a_tiny_announced_mss_is_floored() {
    let _g = fresh();
    let got = sizes(Some(1));
    assert!(got.iter().all(|n| *n <= 88), "{got:?}");
    assert_eq!(got.iter().sum::<usize>(), 1460);
    assert_eq!(got[0], 88);
}

/// The MSS the capsule's parser reads from a SYN carrying `options`.
fn read(options: &[u8]) -> Option<u16> {
    let mut s = Seg::from_peer(80, 1, 0, SYN, &[]);
    s.options = options.to_vec();
    let bytes = encode(&s);
    let (hdr, payload) = crate::tcp::parse(&REMOTE, &LOCAL, &bytes).ok().expect("parses");
    assert!(payload.is_empty());
    hdr.mss
}

#[test]
fn option_areas_are_read_as_stacks_send_them() {
    // Linux: MSS, SACK permitted, timestamps, NOP, window scale.
    let linux = [2, 4, 5, 0xb4, 4, 2, 8, 10, 0, 0, 0, 1, 0, 0, 0, 0, 1, 3, 3, 7];
    assert_eq!(read(&linux), Some(1460));
    assert_eq!(read(&[1, 1, 2, 4, 2, 0x18]), Some(536), "NOPs before it");
    assert_eq!(read(&[3, 3, 7, 2, 4, 5, 0x78]), Some(1400), "after a window scale");
    assert_eq!(read(&[0, 0, 2, 4, 5, 0xb4]), None, "nothing after end of list");
    assert_eq!(read(&[2, 3, 5, 0]), None, "an MSS of the wrong length");
    assert_eq!(read(&[2, 0, 2, 4, 5, 0xb4]), None, "a zero length ends the walk");
    assert_eq!(read(&[2, 1, 2, 4, 5, 0xb4]), None, "so does a length of one");
    assert_eq!(read(&[8, 10, 0, 0, 0, 1, 0, 0]), None, "an option running past the area");
}

fn xorshift(state: &mut u32) -> u32 {
    *state ^= *state << 13;
    *state ^= *state >> 17;
    *state ^= *state << 5;
    *state
}

#[test]
fn random_option_areas_never_step_outside_the_header() {
    let mut s = 0x1234_5678u32;
    for _ in 0..100_000 {
        let words = (xorshift(&mut s) % 11) as usize;
        let mut seg = Seg::from_peer(80, xorshift(&mut s), xorshift(&mut s), ACK, &[]);
        seg.options = (0..words * 4).map(|_| xorshift(&mut s) as u8).collect();
        seg.payload = (0..xorshift(&mut s) % 64).map(|_| xorshift(&mut s) as u8).collect();
        let bytes = encode(&seg);
        let (hdr, payload) = crate::tcp::parse(&REMOTE, &LOCAL, &bytes).ok().expect("parses");
        assert_eq!(payload, seg.payload.as_slice(), "the data starts where the header ends");
        if let Some(mss) = hdr.mss {
            let o = &seg.options;
            assert!(o.windows(4).any(|w| w[0] == 2 && w[1] == 4 && w[2..] == mss.to_be_bytes()));
        }
    }
}
