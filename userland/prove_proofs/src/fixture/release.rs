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

//! A release as the enroll tool builds it: a bootloader tree and a kernel tree
//! of depth 8, each image enrolled at a slot past the first so its path turns
//! both ways, and the record the kernel writes for them with its own encoders.

use nonos_attest_path::{
    boot_context, digest_to_bytes, encode_v4, leaf, pad_leaf, Kind, Poseidon, Tree,
};
use nonos_boot_measure::gate::{BOOT_EPOCH, DEPTH};

use crate::kernel_slots::record::{encode, RECORD_LEN};
use crate::kernel_slots::slot::slot;

pub const LOADER: [u8; 32] = [0x4C; 32];
pub const KERNEL: [u8; 32] = [0x4B; 32];
const LOADER_AT: usize = 5;
const KERNEL_AT: usize = 154;

pub struct Release {
    pub record: [u8; RECORD_LEN],
    pub boot_root: [u8; 32],
    pub kernel_root: [u8; 32],
}

fn tree(kind: Kind, measured: &[u8; 32], at: usize) -> ([u8; 32], Tree) {
    let h = Poseidon::new();
    let mut leaves: Vec<_> = (0..1u32 << DEPTH).map(|i| pad_leaf(&h, &[0x5a; 32], i)).collect();
    leaves[at] = leaf(&h, kind, &boot_context(measured, BOOT_EPOCH)).expect("leaf");
    let tree = Tree::commit(&h, &leaves, DEPTH).expect("tree");
    (digest_to_bytes(&tree.root().expect("root")), tree)
}

/// The v4 trailer of slot `at`; the proof is a stand-in, as the slot is read
/// from the path alone.
fn trailer(t: &Tree, kind: Kind, at: usize) -> Vec<u8> {
    encode_v4(kind, &t.trailer(at).expect("path"), &[0xAB; 96]).expect("v4")
}

pub fn release() -> Release {
    let (boot_root, bt) = tree(Kind::Bootloader, &LOADER, LOADER_AT);
    let (kernel_root, kt) = tree(Kind::Kernel, &KERNEL, KERNEL_AT);
    let b = slot(Kind::Bootloader, &LOADER, &trailer(&bt, Kind::Bootloader, LOADER_AT));
    let k = slot(Kind::Kernel, &KERNEL, &trailer(&kt, Kind::Kernel, KERNEL_AT));
    let (b, k) = (b.expect("loader slot"), k.expect("kernel slot"));
    assert_eq!((b.root, k.root), (boot_root, kernel_root), "the kernel's roots are the trees'");
    Release { record: encode(&b, &k), boot_root, kernel_root }
}
