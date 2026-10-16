// NONOS Operating System (AGPL-3.0-or-later)
//! A saved passphrase is one the network's kind of security can use: WPA2's
//! 8 to 63 bytes or 64 hex digits unless the network is pinned to WPA3, then
//! SAE's 1 to 64 bytes. Empty is refused for both, and a refused save leaves
//! the list as it was, byte for byte.

use super::list::{SavedList, PLAIN_LEN};
use crate::join_wire::JoinFlags;

const PIN: JoinFlags = JoinFlags { wpa3_only: true, hidden: false };
const PLAIN: JoinFlags = JoinFlags { wpa3_only: false, hidden: false };

fn plain(l: &SavedList) -> Vec<u8> {
    let mut out = [0u8; PLAIN_LEN];
    l.encode(&mut out);
    out.to_vec()
}

/// Whether `pass` is taken for a network saved with `flags`, and that a
/// refusal changes nothing.
fn takes(flags: JoinFlags, pass: &[u8]) -> bool {
    let mut l = SavedList::new();
    assert!(l.put_with(b"other", b"password123", PLAIN));
    let before = plain(&l);
    let taken = l.put_dropping(b"net", pass, flags).is_some();
    if !taken {
        assert_eq!(plain(&l), before, "a refused save changed the list");
    }
    taken
}

#[test]
fn a_wpa2_passphrase_is_8_to_63_bytes_or_64_hex_digits() {
    let hex64 = [b'a'; 64];
    let mut not_hex = [b'a'; 64];
    not_hex[63] = b'g';
    let cases: [(&[u8], bool); 7] = [
        (&[b'p'; 7], false),
        (&[b'p'; 8], true),
        (&[b'p'; 63], true),
        (&hex64, true),
        (&not_hex, false),
        (&[b'p'; 65], false),
        (b"", false),
    ];
    for (pass, ok) in cases {
        assert_eq!(takes(PLAIN, pass), ok, "{} bytes", pass.len());
    }
}

#[test]
fn a_wpa3_password_is_1_to_64_bytes_and_never_empty() {
    let cases: [(&[u8], bool); 5] =
        [(b"", false), (b"x", true), (&[b'p'; 7], true), (&[b'g'; 64], true), (&[b'p'; 65], false)];
    for (pass, ok) in cases {
        assert_eq!(takes(PIN, pass), ok, "{} bytes", pass.len());
    }
}

#[test]
fn a_network_pinned_before_keeps_its_wpa3_rule_when_saved_again() {
    let mut l = SavedList::new();
    assert!(l.put_with(b"net", b"short", PIN));
    assert!(l.put_with(b"net", b"tiny", PLAIN), "still pinned, so SAE's rule applies");
    assert!(!l.put_with(b"net", b"", PLAIN), "and empty is still refused");
    assert_eq!(l.passphrase(l.find(b"net").unwrap()), b"tiny");
}
