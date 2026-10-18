/*
 * NONOS Operating System (AGPL-3.0-or-later)
 */

//! A save that drops a network to make room names it, and the sealed record
//! written after it, opened again, holds nothing of it.

use super::file::{open_file, seal_file, FILE_LEN};
use super::list::{SavedList, SLOTS};
use crate::wifi::join_wire::JoinFlags;

#[test]
fn the_record_sealed_after_a_drop_holds_nothing_of_the_dropped_network() {
    let (key, nonce) = ([7u8; 32], [9u8; 12]);
    let pin = JoinFlags { wpa3_only: true, hidden: false };
    let mut l = SavedList::new();
    assert!(l.put_with(b"pinned-net", b"pinned-pass", pin));
    for n in 1..SLOTS as u8 {
        assert!(l.put(&[b'n', b'e', b't', b'0' + n], b"12345678"));
    }
    let gone = l.put_dropping(b"newest", b"87654321", JoinFlags::default());
    assert_eq!(gone.flatten().map(|d| d.ssid().to_vec()), Some(b"net1".to_vec()));
    let mut file = [0u8; FILE_LEN];
    assert!(seal_file(&l, &key, &nonce, &mut file));
    let back = open_file(&file, &key).ok().unwrap();
    assert!((0..back.len()).all(|i| back.ssid(i) != b"net1"));
    assert_eq!(
        (back.len(), back.ssid(0), back.join_flags(0).wpa3_only),
        (SLOTS, &b"pinned-net"[..], true)
    );
}
