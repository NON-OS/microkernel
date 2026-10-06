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

//! A slot as the kernel builds it is the slot the enroll tool committed, the
//! one the device circuit's witness takes.

use nonos_attest_path::{boot_context, context_digest, Kind};

use super::footer::regions;
use super::slot::{slot, KIND_BOOTLOADER, KIND_KERNEL, PATH_LEN};
use crate::fixture::{measure, signed_file, trailer, tree, EPOCH, HAS_ZK_PROOF};

#[test]
fn the_kernel_slot_out_of_its_signed_file_is_the_enrolled_one() {
    let kernel = vec![0x7F; 8192];
    let (root, t) = tree(Kind::Kernel, &[[0x11; 32], measure(&kernel)]);
    let f = signed_file(&kernel, &trailer(&t, Kind::Kernel, 1), HAS_ZK_PROOF, 2);
    let r = regions(&f).expect("regions");
    let s = slot(Kind::Kernel, &measure(r.kernel), r.proof).expect("slot");
    assert_eq!((s.kind, s.epoch, s.measurement), (KIND_KERNEL, EPOCH, measure(&kernel)));
    assert_eq!(Some(s.digest), context_digest(&boot_context(&measure(&kernel), EPOCH)));
    assert_eq!(s.root, root, "the root the enroll tool committed");
    assert_eq!(s.path[..], t.trailer(1).expect("path")[..]);
}

#[test]
fn the_bootloader_slot_is_the_enrolled_one() {
    let m = [0x33; 32];
    let (root, t) = tree(Kind::Bootloader, &[m]);
    let s = slot(Kind::Bootloader, &m, &trailer(&t, Kind::Bootloader, 0)).expect("slot");
    assert_eq!((s.kind, s.root), (KIND_BOOTLOADER, root));
    assert_eq!(s.path.len(), PATH_LEN);
}

#[test]
fn a_kernel_changed_after_enrollment_names_another_root() {
    let kernel = vec![0x7F; 4096];
    let (root, t) = tree(Kind::Kernel, &[measure(&kernel)]);
    let mut changed = kernel.clone();
    changed[100] ^= 1;
    let s = slot(Kind::Kernel, &measure(&changed), &trailer(&t, Kind::Kernel, 0)).expect("slot");
    assert_ne!(s.root, root, "a proof under it would be refused");
}

#[test]
fn a_trailer_of_another_kind_or_damaged_is_no_slot() {
    let m = [0x22; 32];
    let (_, t) = tree(Kind::Kernel, &[m]);
    let v4 = trailer(&t, Kind::Kernel, 0);
    assert_eq!(slot(Kind::Bootloader, &m, &v4), None, "a kernel's trailer is not a loader's");
    assert_eq!(slot(Kind::Capsule, &m, &v4), None);
    assert_eq!(slot(Kind::Pad, &m, &v4), None);
    for cut in 0..v4.len() {
        assert_eq!(slot(Kind::Kernel, &m, &v4[..cut]), None, "cut at {cut}");
    }
}
