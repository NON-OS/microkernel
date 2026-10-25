// NONOS Operating System (AGPL-3.0-or-later)
//! A full list makes room by dropping its oldest network not pinned to WPA3,
//! and a pinned one only when all four are pinned; the save names what it
//! dropped, and the list keeps no trace of it.

use super::list::{SavedList, PLAIN_LEN};
use crate::join_wire::JoinFlags;

const PIN: JoinFlags = JoinFlags { wpa3_only: true, hidden: false };
const PLAIN: JoinFlags = JoinFlags { wpa3_only: false, hidden: false };

fn full(flags: [JoinFlags; 4]) -> SavedList {
    let mut l = SavedList::new();
    for (name, f) in [b"net-a", b"net-b", b"net-c", b"net-d"].iter().zip(flags) {
        assert_eq!(l.put_dropping(*name, b"password123", f).map(|d| d.is_none()), Some(true));
    }
    l
}

fn names(l: &SavedList) -> Vec<&[u8]> {
    (0..l.len()).map(|i| l.ssid(i)).collect()
}

fn save(l: &mut SavedList, ssid: &[u8]) -> Vec<u8> {
    let gone = l.put_dropping(ssid, b"password123", PLAIN).expect("taken").expect("one dropped");
    gone.ssid().to_vec()
}

#[test]
fn the_oldest_unpinned_network_goes_whichever_comes_first() {
    // An unpinned network oldest: it goes.
    let mut l = full([PLAIN, PIN, PLAIN, PIN]);
    assert_eq!(save(&mut l, b"net-e"), b"net-a");
    assert_eq!(names(&l), [&b"net-b"[..], b"net-c", b"net-d", b"net-e"]);
    // A pinned network oldest: the first unpinned after it goes instead.
    let mut l = full([PIN, PLAIN, PIN, PLAIN]);
    assert_eq!(save(&mut l, b"net-e"), b"net-b");
    assert_eq!(names(&l), [&b"net-a"[..], b"net-c", b"net-d", b"net-e"]);
    assert_eq!(save(&mut l, b"net-f"), b"net-d", "the pins outlive every plain save");
    assert!(l.join_flags(0).wpa3_only && l.join_flags(1).wpa3_only);
}

#[test]
fn a_pin_is_dropped_only_when_all_four_are_pinned() {
    let mut l = full([PIN; 4]);
    assert_eq!(save(&mut l, b"net-e"), b"net-a");
    assert_eq!(names(&l), [&b"net-b"[..], b"net-c", b"net-d", b"net-e"]);
}

#[test]
fn a_save_that_needs_no_room_drops_nothing_and_a_refused_one_changes_nothing() {
    let mut l = full([PLAIN; 4]);
    let again = l.put_dropping(b"net-b", b"new-password", PLAIN);
    assert_eq!(again.map(|d| d.is_none()), Some(true), "saved again in place of itself");
    assert_eq!(l.len(), 4);
    assert!(l.put_dropping(b"", b"password123", PLAIN).is_none(), "an empty name");
    assert!(l.put_dropping(&[b'x'; 33], b"password123", PLAIN).is_none(), "a name too long");
    assert!(l.put_dropping(b"net-e", &[b'p'; 65], PLAIN).is_none(), "a passphrase too long");
    assert_eq!(names(&l), [&b"net-a"[..], b"net-c", b"net-d", b"net-b"]);
}

#[test]
fn the_dropped_network_leaves_no_trace_in_the_list() {
    let mut l = full([PLAIN, PIN, PIN, PIN]);
    assert_eq!(save(&mut l, b"net-e"), b"net-a");
    let mut plain = [0u8; PLAIN_LEN];
    l.encode(&mut plain);
    assert!(!plain.windows(5).any(|w| w == b"net-a"), "its name is in no slot");
    assert!(l.find(b"net-a").is_none());
}
