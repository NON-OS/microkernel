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

//! The SYN profile: what the stack's opening segments carry on the wire, in
//! the shipped profile and the normalised one, and that the builder lays an
//! options area out so the stack's own reader reads it back.

use crate::peer::{connect, fresh, sent, SYN};
use crate::tcp::syn_profile::{syn_options, syn_window, NORMALISED};
use crate::tcp::{build, parse, BuildRequest, FLAG_SYN};

const SRC: [u8; 4] = [10, 0, 2, 15];
const DST: [u8; 4] = [93, 184, 216, 34];

fn syn_bytes(normalised: bool) -> Vec<u8> {
    let (area, len) = syn_options(normalised, 1460);
    let req = BuildRequest {
        src: SRC,
        dst: DST,
        src_port: 49152,
        dst_port: 443,
        seq: 0x1122_3344,
        ack: 0,
        flags: FLAG_SYN,
        window: syn_window(normalised),
        options: &area[..len],
        payload: &[],
    };
    let mut out = [0u8; 64];
    let n = build(&req, &mut out).ok().expect("build");
    out[..n].to_vec()
}

#[test]
fn the_shipped_syn_carries_no_options_and_a_window_of_8192() {
    let b = syn_bytes(false);
    assert_eq!(b.len(), 20);
    assert_eq!(b[12] >> 4, 5);
    assert_eq!(u16::from_be_bytes([b[14], b[15]]), 8192);
    let (h, _) = parse(&SRC, &DST, &b).ok().expect("parse");
    assert_eq!(h.mss, None);
}

#[test]
fn the_normalised_syn_carries_the_mss_and_a_window_of_29200() {
    let b = syn_bytes(true);
    assert_eq!(b.len(), 24);
    assert_eq!(b[12] >> 4, 6, "data offset counts the option word");
    assert_eq!(&b[20..24], &[2, 4, 0x05, 0xb4]);
    assert_eq!(u16::from_be_bytes([b[14], b[15]]), 29200);
    let (h, payload) = parse(&SRC, &DST, &b).ok().expect("its own reader accepts it");
    assert_eq!(h.mss, Some(1460));
    assert_eq!(h.window, 29200);
    assert!(payload.is_empty());
}

#[test]
fn the_normalised_window_fits_what_the_receive_queue_holds() {
    // Advertising more than RX_DEPTH segments would invite data the queue
    // then has to drop.
    assert!(syn_window(true) <= crate::tcp::RWND_MAX);
    assert!(syn_window(false) <= crate::tcp::RWND_MAX);
}

#[test]
fn options_that_are_not_whole_words_or_too_long_are_refused() {
    let mut out = [0u8; 128];
    for bad in [&[2u8, 4, 5][..], &[1u8; 44][..]] {
        let req = BuildRequest {
            src: SRC,
            dst: DST,
            src_port: 1,
            dst_port: 2,
            seq: 0,
            ack: 0,
            flags: FLAG_SYN,
            window: 0,
            options: bad,
            payload: &[],
        };
        assert!(build(&req, &mut out).is_err());
    }
}

#[test]
fn the_syn_the_capsule_sends_is_the_profile_it_was_built_with() {
    let _g = fresh();
    let _ = connect();
    let syn = sent().into_iter().find(|s| s.flags & SYN != 0).expect("a SYN went out");
    let (area, len) = syn_options(NORMALISED, 1460);
    assert_eq!(syn.options, area[..len].to_vec());
    assert_eq!(syn.window, syn_window(NORMALISED));
}

#[test]
fn print_the_syn_bytes_for_the_branch_notes() {
    for normalised in [false, true] {
        let hex: String = syn_bytes(normalised).iter().map(|b| format!("{b:02x}")).collect();
        println!("syn normalised={normalised} {hex}");
    }
}

#[test]
fn what_each_syn_makes_a_peer_send_for_30_mb() {
    // A peer sizes its segments by the MSS option in the SYN it was sent,
    // and by 536 when there is none (RFC 9293 3.7.1). Read here from the
    // stack's own SYN bytes by the stack's own option reader.
    const BYTES: usize = 30 * 1024 * 1024;
    for normalised in [false, true] {
        let b = syn_bytes(normalised);
        let (h, _) = parse(&SRC, &DST, &b).ok().expect("parse");
        let mss = usize::from(h.mss.unwrap_or(536));
        let segments = BYTES.div_ceil(mss);
        println!("syn normalised={normalised}: peer segment {mss} B, {segments} segments for 30 MiB");
        assert_eq!(mss, if normalised { 1460 } else { 536 });
    }
}
