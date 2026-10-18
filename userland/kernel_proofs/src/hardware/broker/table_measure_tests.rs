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

//! The firmware tables' measurement (F-90): each record's bytes, the list
//! in hand-out order with an identical repeat skipped and a changed table
//! kept, and one digest over the domain, the count and every record that
//! any change of order, count, table or byte moves.

use super::acpi::table_measure::{digest, note, Measured, DOMAIN, RECORD_LEN};

fn t(sig: &[u8; 4], instance: u32, body: &[u8]) -> Measured {
    Measured::of(*sig, instance, body)
}

#[test]
fn a_record_is_its_signature_instance_length_and_digest() {
    let m = t(b"SSDT", 3, b"some aml");
    let e = m.encode();
    assert_eq!(e.len(), RECORD_LEN);
    assert_eq!(&e[..4], b"SSDT");
    assert_eq!(e[4..8], 3u32.to_be_bytes());
    assert_eq!(e[8..12], 8u32.to_be_bytes());
    assert_eq!(e[12..], *blake3::hash(b"some aml").as_bytes());
}

#[test]
fn the_list_keeps_load_order_and_every_change() {
    let mut list = Vec::new();
    let (dsdt, ssdt) = (t(b"DSDT", 0, b"a"), t(b"SSDT", 0, b"b"));
    assert!(note(&mut list, dsdt));
    assert!(note(&mut list, ssdt));
    assert!(!note(&mut list, dsdt), "the same bytes again add nothing");
    let changed = t(b"DSDT", 0, b"A");
    assert!(note(&mut list, changed), "changed bytes are a new record");
    assert_eq!(list, [dsdt, ssdt, changed]);
}

#[test]
fn the_digest_folds_domain_count_and_records_in_order() {
    let (a, b, c) = (t(b"DSDT", 0, b"x"), t(b"SSDT", 0, b"y"), t(b"SSDT", 1, b"y"));
    let mut h = blake3::Hasher::new();
    h.update(DOMAIN);
    h.update(&2u32.to_be_bytes());
    h.update(&a.encode());
    h.update(&b.encode());
    assert_eq!(digest(&[a, b]), *h.finalize().as_bytes());
    assert_ne!(digest(&[a, b]), digest(&[b, a]), "order counts");
    assert_ne!(digest(&[b]), digest(&[c]), "the instance counts");
    assert_ne!(digest(&[a]), digest(&[a, a]), "the count counts");
    assert_ne!(digest(&[]), digest(&[a]));
    let flipped = t(b"DSDT", 0, b"z");
    assert_ne!(digest(&[a, b]), digest(&[flipped, b]), "a byte of a table counts");
}
