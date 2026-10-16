// NONOS Operating System (AGPL-3.0-or-later)
//! The scan request is a passive one-iteration scan of the given channels.

use crate::gen3::scan::{complete, request};

#[test]
fn the_scan_request_is_a_passive_one_iteration_scan_of_the_given_channels() {
    let r = request(&[1, 6, 36, 165], 0).expect("builds");
    assert_eq!(r.len(), 1940);
    let u16at = |o: usize| u16::from_le_bytes([r[o], r[o + 1]]);
    let u32at = |o: usize| u32::from_le_bytes(r[o..o + 4].try_into().unwrap());
    assert_eq!((u32at(0), u32at(4)), (0, 6), "uid 0, out-of-channel priority 6");
    assert_eq!(u16at(8), (1 << 11) | (1 << 7) | (1 << 1), "passive, adaptive dwell, pass all");
    assert_eq!(r[11], 0, "link 0");
    assert_eq!((r[12], r[13], r[14], r[15], r[16]), (10, 10, 2, 8, 10));
    assert_eq!(u16at(18), 300, "full-scan budget");
    assert_eq!((u32at(20), u32at(28)), (0, 0), "unassociated: no time limits");
    assert_eq!(u32at(36), 6);
    assert_eq!((r[40], r[41]), (110, 110), "passive dwell");
    assert_eq!((r[44], r[45], r[46], r[47]), (0x20, 4, 10, 2), "channel order, count, n_aps overrides");
    let ch = |i: usize| (u32at(48 + i * 8), r[48 + i * 8 + 4], r[48 + i * 8 + 6]);
    assert_eq!(ch(0), (1 << 30, 1, 1), "2.4 GHz band, one iteration");
    assert_eq!(ch(1), (1 << 30, 6, 1));
    assert_eq!(ch(2), (0, 36, 1), "5 GHz band is 0");
    assert_eq!(ch(3), (0, 165, 1));
    assert_eq!((u16at(584), r[586]), (0, 1), "one plan, one iteration");
    assert!(r[596..].iter().all(|&b| b == 0), "no probe request, no SSIDs");
    assert!(request(&[], 0).is_none());
    assert!(request(&[1; 68], 0).is_none(), "more channels than the request holds");
    assert_eq!(request(&[1], 3).unwrap()[..4], 3u32.to_le_bytes(), "the uid it is given");
    let mut done = vec![0u8; 16];
    done[6] = 1;
    assert_eq!(complete(&done), Some((0, 1)));
    assert_eq!(complete(&done[..15]), None);
}
