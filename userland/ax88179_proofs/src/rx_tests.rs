// NONOS Operating System (AGPL-3.0-or-later)
//! Receive transfers in ax88179_rx_fixup's layout: good ones split into
//! frames, bad frames skipped, hostile counts and offsets dropped whole.

use crate::ax::rx::{frames, RXHDR_CRC_ERR, RXHDR_DROP_ERR};
use crate::layout::{frame, rx_transfer};

fn parse(xfer: &[u8]) -> (bool, Vec<Vec<u8>>) {
    let mut spans = Vec::new();
    let ok = frames(xfer, &mut spans);
    (ok, spans.iter().map(|&(at, len)| xfer[at..at + len].to_vec()).collect())
}

#[test]
fn frames_come_out_whole_with_and_without_dummy_headers() {
    let (a, b) = (frame(60, 0xa1), frame(1514, 0xb2));
    for dummies in [true, false] {
        // An L4 type of TCP (16) in the flags is checksum news only.
        let xfer = rx_transfer(&[(&a, 16), (&b, 0)], dummies);
        assert_eq!(xfer.len() % 8, 0);
        assert_eq!(parse(&xfer), (true, vec![a.clone(), b.clone()]));
    }
}

#[test]
fn errored_runt_and_oversized_frames_are_skipped_and_the_rest_kept() {
    let (good, runt, vlan) = (frame(64, 7), frame(13, 8), frame(1518, 9));
    let set =
        [(&good[..], RXHDR_CRC_ERR), (&good, RXHDR_DROP_ERR), (&runt, 0), (&vlan, 0), (&good, 0)];
    assert_eq!(parse(&rx_transfer(&set, true)), (true, vec![good.clone()]));
    assert_eq!(parse(&rx_transfer(&[], true)), (true, vec![]), "count 0");
}

#[test]
fn hostile_counts_offsets_and_lengths_drop_the_transfer() {
    let good = rx_transfer(&[(&frame(60, 1), 0)], true);
    let trailer = |x: &mut Vec<u8>, v: u32| {
        let n = x.len();
        x[n - 4..].copy_from_slice(&v.to_le_bytes());
    };
    let off = u32::from_le_bytes(good[good.len() - 4..].try_into().unwrap()) >> 16;
    for hostile in [0xffff | off << 16, 2 | 0xfff0 << 16, 64 | off << 16, 2 | 0xffff << 16] {
        let mut x = good.clone();
        trailer(&mut x, hostile);
        assert_eq!(parse(&x), (false, vec![]), "{hostile:#010x}");
    }
    let mut long = good.clone();
    long[off as usize..off as usize + 4].copy_from_slice(&(0x1fffu32 << 16).to_le_bytes());
    assert_eq!(parse(&long), (false, vec![]), "a packet running into the headers");
    for short in 0..4 {
        assert!(!parse(&good[..short]).0);
    }
}

#[test]
fn a_bad_entry_after_good_ones_drops_the_whole_transfer() {
    let mut x = rx_transfer(&[(&frame(60, 1), 0), (&frame(60, 2), 0)], false);
    let off = (u32::from_le_bytes(x[x.len() - 4..].try_into().unwrap()) >> 16) as usize;
    x[off + 4..off + 8].copy_from_slice(&(200u32 << 16).to_le_bytes());
    assert_eq!(parse(&x), (false, vec![]));
}
