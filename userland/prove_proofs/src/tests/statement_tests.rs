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

//! The statement is the one its parts give, field by field, and the witness
//! is this device's; after `wipe` nothing of the witness is left.

use nonos_device_attest::{ek_id, scope, tag, words_of};

use crate::abi::parse_boot_slots;
use crate::assemble::wipe::wipe;
use crate::fixture::{area, device, request, root_bytes, DEPTH, NONCE, VERIFIER, WINDOW};

#[test]
fn the_statement_is_the_one_its_parts_give() {
    let d = device();
    let (st, w) = d.assemble().expect("an enrolled device");
    let e = scope(VERIFIER, WINDOW).expect("scope");
    let s = words_of(&d.secret);
    assert_eq!(st.boot_root, words_of(&d.release.boot_root));
    assert_eq!(st.kernel_root, words_of(&d.release.kernel_root));
    assert_eq!((st.device_root, st.device_depth), (d.registry.root(), DEPTH));
    assert_eq!((st.scope, st.context, st.tag), (e, words_of(&NONCE), tag(&s, &e)));
    assert_eq!(w.secret, s);
    let path = d.registry.path(&ek_id(area(&d.eks[1]))).expect("enrolled");
    assert_eq!((w.device.siblings, w.device.right), (path.siblings, path.right));
    let slots = parse_boot_slots(&d.record).expect("record");
    assert_eq!(w.bootloader.digest, slots.bootloader.digest);
    assert_eq!(w.kernel.digest, slots.kernel.digest);
    assert_eq!(w.bootloader.path.right.len(), 8);
    assert!(w.kernel.path.right.iter().any(|&r| r) && w.kernel.path.right.iter().any(|&r| !r));
}

/// Another verifier is another scope, and the same device has another tag in
/// it; the secret and the registry do not change.
#[test]
fn another_verifier_gives_another_tag() {
    let mut d = device();
    let (a, _) = d.assemble().expect("first");
    d.request = request(b"relay.example", WINDOW, &NONCE, &root_bytes(&d.registry));
    let (b, _) = d.assemble().expect("second");
    assert_ne!((a.scope, a.tag), (b.scope, b.tag));
    assert_eq!((a.device_root, a.boot_root), (b.device_root, b.boot_root));
    d.request = request(VERIFIER, WINDOW + 1, &NONCE, &root_bytes(&d.registry));
    let (c, _) = d.assemble().expect("next window");
    assert_ne!(a.tag, c.tag);
}

#[test]
fn wipe_leaves_nothing_of_the_witness() {
    let (_, mut w) = device().assemble().expect("an enrolled device");
    wipe(&mut w);
    let zero = words_of(&[0; 32]);
    assert_eq!(w.secret, zero);
    for slot in [&w.bootloader, &w.kernel] {
        assert_eq!(slot.digest, [0; 32]);
        assert!(slot.path.siblings.iter().all(|s| *s == zero));
        assert!(slot.path.right.iter().all(|r| !r));
    }
    assert!(w.device.siblings.iter().all(|s| *s == zero) && w.device.right.iter().all(|r| !r));
}
