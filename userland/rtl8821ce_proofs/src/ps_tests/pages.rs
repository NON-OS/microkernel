// NONOS Operating System (AGPL-3.0-or-later)
//! The frames the firmware sends asleep and the reserved pages that hold
//! them, byte for byte as mac80211 and rtw88 lay them out.

use crate::h2c::cmds::{media_status, rsvd_page};
use crate::ps::frames::{null, ps_poll, qos_null};
use crate::ps::pages::{sta_pages, LEN, NULL_PAGE, PAGE, PS_POLL_PAGE, QOS_NULL_PAGE};

const AP: [u8; 6] = [0x02, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE];
const STA: [u8; 6] = [0x02, 0x11, 0x22, 0x33, 0x44, 0x55];

fn word(b: &[u8], at: usize, w: usize) -> u32 {
    u32::from_le_bytes(b[at + 4 * w..at + 4 * w + 4].try_into().unwrap())
}

#[test]
fn the_frames_are_mac80211s() {
    let p = ps_poll(5, AP, STA);
    assert_eq!((&p[..4], &p[4..10], &p[10..]), (&[0xA4, 0, 5, 0xC0][..], &AP[..], &STA[..]));
    let n = null(AP, STA);
    assert_eq!(
        (&n[..4], &n[4..10], &n[10..16], &n[16..22]),
        (&[0x48, 1, 0, 0][..], &AP[..], &STA[..], &AP[..])
    );
    let q = qos_null(AP, STA);
    assert_eq!((q[0], q[1], &q[24..]), (0xC8, 1, &[7, 0][..]), "QoS Null at TID 7");
}

#[test]
fn each_frame_sits_a_page_in_behind_its_own_descriptor() {
    let b = sta_pages(5, AP, STA);
    assert_eq!((LEN, PAGE), (464, 128), "four pages, the last cut after its frame");
    let at = |page: u8| 80 + PAGE * (usize::from(page) - 1);
    for (page, frame, len) in [
        (PS_POLL_PAGE, &ps_poll(5, AP, STA)[..], 16),
        (QOS_NULL_PAGE, &qos_null(AP, STA)[..], 26),
        (NULL_PAGE, &null(AP, STA)[..], 24),
    ] {
        let d = at(page);
        assert_eq!(&b[d + 48..d + 48 + len], frame, "page {page}");
        let ps_poll = page == PS_POLL_PAGE;
        let seq = if ps_poll { 0 } else { 1 << 31 };
        assert_eq!(word(&b, d, 0), len as u32 | 48 << 16 | 1 << 26 | seq, "length, offset, LS");
        assert_eq!(word(&b, d, 1), 18 << 8 | 8 << 16, "management queue, 1 Mb/s rate id");
        assert_eq!(word(&b, d, 2), if page == QOS_NULL_PAGE { 1 << 23 } else { 0 });
        let nav = if ps_poll { 1 << 15 } else { 0 };
        assert_eq!(word(&b, d, 3), 1 << 8 | 1 << 10 | nav, "fixed rate, no fallback");
        assert_eq!(word(&b, d, 8), if ps_poll { 0 } else { 1 << 15 }, "hardware sequence");
    }
    assert!(b[..80].iter().all(|&x| x == 0), "page 0 is the one-byte stand-in");
}

#[test]
fn the_commands_name_the_pages_and_the_station() {
    assert_eq!(rsvd_page(PS_POLL_PAGE, NULL_PAGE, QOS_NULL_PAGE), (0x0301_0000, 2));
    assert_eq!(media_status(true, 0), (0x0101, 0));
    assert_eq!(media_status(false, 3), (0x0003_0001, 0));
}
