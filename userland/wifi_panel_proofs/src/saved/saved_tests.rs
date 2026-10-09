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

//! Proofs for the saved network list's flags: a network saved as WPA3 stays
//! WPA3 (saving it again, even without the flag, cannot open it to WPA2),
//! the hidden mark is kept the same way, the flags ride in the top bits of
//! the SSID length octet so a record written before them reads unchanged, and
//! a record whose lengths no slot could have been written with is refused
//! whole rather than half-read.

use super::list::{SavedList, PLAIN_LEN, SLOT_LEN};
use crate::join_wire::JoinFlags;

const WPA3: JoinFlags = JoinFlags { wpa3_only: true, hidden: false };
const HIDDEN: JoinFlags = JoinFlags { wpa3_only: false, hidden: true };

fn round_trip(list: &SavedList) -> SavedList {
    let mut raw = [0u8; PLAIN_LEN];
    list.encode(&mut raw);
    SavedList::decode(&raw).expect("a list this client wrote decodes")
}

#[test]
fn a_plain_save_has_no_flags() {
    let mut l = SavedList::new();
    assert!(l.put(b"Home", b"password123"));
    assert_eq!(l.join_flags(0), JoinFlags::default());
    assert_eq!(l.ssid(0), b"Home");
}

#[test]
fn the_flags_survive_the_record() {
    let mut l = SavedList::new();
    assert!(l.put_with(b"Home", b"password123", WPA3));
    assert!(l.put_with(&[b'h'; 32], b"secret-pass", HIDDEN));
    let back = round_trip(&l);
    assert_eq!(back.len(), 2);
    assert_eq!((back.ssid(0), back.join_flags(0)), (&b"Home"[..], WPA3));
    assert_eq!(back.ssid(1), &[b'h'; 32][..], "a 32-octet name is not cut by the flag bits");
    assert_eq!(back.join_flags(1), HIDDEN);
    assert_eq!(back.passphrase(1), b"secret-pass");
}

#[test]
fn a_network_saved_as_wpa3_stays_wpa3_when_saved_again() {
    let mut l = SavedList::new();
    assert!(l.put_with(b"Home", b"old-password", WPA3));
    assert!(l.put(b"Home", b"new-password"), "a new passphrase, saved without flags");
    let i = l.find(b"Home").expect("still saved, once");
    assert_eq!(l.len(), 1);
    assert!(l.join_flags(i).wpa3_only, "never opened to WPA2");
    assert_eq!(l.passphrase(i), b"new-password");
    assert!(l.put_with(b"Home", b"x-password", HIDDEN));
    let i = l.find(b"Home").unwrap();
    assert_eq!(l.join_flags(i), JoinFlags { wpa3_only: true, hidden: true }, "flags only add");
}

#[test]
fn forgetting_clears_the_flags() {
    let mut l = SavedList::new();
    assert!(l.put_with(b"Home", b"password123", WPA3));
    l.remove(0);
    assert!(l.is_empty());
    assert!(l.put(b"Home", b"password123"));
    assert_eq!(l.join_flags(0), JoinFlags::default(), "saved afresh, as the person chose");
}

#[test]
fn a_record_written_before_the_flags_reads_unchanged() {
    // `[ssid_len][pass_len][ssid 32][pass 64]` with plain lengths.
    let mut raw = [0u8; PLAIN_LEN];
    raw[0] = 4;
    raw[1] = 8;
    raw[2..6].copy_from_slice(b"Cafe");
    raw[34..42].copy_from_slice(b"espresso");
    let l = SavedList::decode(&raw).expect("decodes");
    assert_eq!((l.len(), l.ssid(0), l.passphrase(0)), (1, &b"Cafe"[..], &b"espresso"[..]));
    assert_eq!(l.join_flags(0), JoinFlags::default());
}

#[test]
fn records_no_slot_could_hold_are_refused_whole() {
    let mut good = SavedList::new();
    assert!(good.put(b"Home", b"password123"));
    let mut raw = [0u8; PLAIN_LEN];
    good.encode(&mut raw);
    let mut flags_no_name = raw;
    flags_no_name[SLOT_LEN] = 0x80; // a second slot: flags but an empty name
    assert!(SavedList::decode(&flags_no_name).is_none());
    let mut long_name = raw;
    long_name[0] = 0x80 | 33;
    assert!(SavedList::decode(&long_name).is_none(), "33 octets under the flag bits");
    let mut long_pass = raw;
    long_pass[1] = 65;
    assert!(SavedList::decode(&long_pass).is_none());
    assert!(SavedList::decode(&raw[..PLAIN_LEN - 1]).is_none(), "a short record");
}
