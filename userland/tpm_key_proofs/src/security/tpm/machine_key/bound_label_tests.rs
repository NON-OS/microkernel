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

//! Machine keys bound to their caller. The key is TPM2_HMAC over the label
//! the kernel hands the TPM, so these show what that label is: different for
//! two namespaces, out of reach of a caller holding another namespace, and
//! the caller's own bytes, unchanged, for every unbound label.

use super::bound_label::{bound_label, BOUND_LABEL_MAX, BOUND_PREFIX};
use super::consts::LABEL_MAX;

const EPERM: i32 = 1;
const EINVAL: i32 = 22;
/// Stand-ins for BLAKE3 of two manifest namespaces.
const METER: [u8; 32] = [0x6d; 32];
const LINUX: [u8; 32] = [0x4c; 32];

fn bound(name: &[u8]) -> Vec<u8> {
    [&[BOUND_PREFIX][..], name].concat()
}

#[test]
fn two_namespaces_naming_one_bound_label_get_different_keys() {
    let label = bound(b"nonos.meter.root.v1");
    let meter = bound_label(&label, Some(&METER)).unwrap();
    let linux = bound_label(&label, Some(&LINUX)).unwrap();
    assert_ne!(meter, linux);
    assert_eq!(meter, [label.as_slice(), &METER].concat());
    assert!(meter.len() <= LABEL_MAX);
}

#[test]
fn a_linux_shaped_caller_cannot_name_the_meters_key() {
    let meter = bound_label(&bound(b"nonos.meter.root.v1"), Some(&METER)).unwrap();
    // Whatever it names, the meter's label or its hash or key bytes, the
    // kernel appends the caller's own namespace, never the meter's.
    for name in [&b"nonos.meter.root.v1"[..], &METER[..31], &meter[1..BOUND_LABEL_MAX]] {
        let got = bound_label(&bound(name), Some(&LINUX)).unwrap();
        assert_ne!(got, meter);
        assert_eq!(&got[got.len() - 32..], &LINUX);
    }
}

#[test]
fn a_caller_the_vendor_did_not_prove_gets_eperm() {
    assert_eq!(bound_label(&bound(b"nonos.meter.root.v1"), None), Err(EPERM));
    assert_eq!(bound_label(&[BOUND_PREFIX], None), Err(EPERM));
    let long = bound(&[7; LABEL_MAX]);
    assert_eq!(bound_label(&long, None), Err(EPERM), "no length hint first");
    assert_eq!(bound_label(&long, Some(&METER)), Err(EINVAL));
}

#[test]
fn unbound_labels_are_unchanged_byte_for_byte() {
    let labels: [&[u8]; 5] =
        [b"nonos.vault.root.v1", b"wifi/saved-networks", b"x", &[2, 1, 1], &[0xff; LABEL_MAX]];
    for l in labels {
        assert_eq!(bound_label(l, None).unwrap(), l);
        assert_eq!(bound_label(l, Some(&METER)).unwrap(), l);
    }
}
