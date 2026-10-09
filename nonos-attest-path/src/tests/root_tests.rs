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

use super::fixture::{capsule_ctx, kernel_ctx, policy, DEPTH, EPOCH};
use crate::leaf::Kind;
use crate::root::root_of;
use crate::verify::verify;

#[test]
fn every_enrolled_slot_folds_to_the_tree_root() {
    let (root, tree, cap, kernel) = policy();
    let t0 = tree.trailer(0).expect("trailer 0");
    let t1 = tree.trailer(1).expect("trailer 1");
    let ctx0 = capsule_ctx(&cap, 0x7, EPOCH);
    let ctx1 = kernel_ctx(&kernel, EPOCH);
    assert_eq!(root_of(DEPTH, Kind::Capsule, &ctx0, &t0), Some(root));
    assert_eq!(root_of(DEPTH, Kind::Kernel, &ctx1, &t1), Some(root));
}

#[test]
fn what_root_of_names_is_what_verify_admits_under() {
    let (_, tree, _, kernel) = policy();
    let t1 = tree.trailer(1).expect("trailer 1");
    let ctx = kernel_ctx(&kernel, EPOCH + 1);
    let named = root_of(DEPTH, Kind::Kernel, &ctx, &t1).expect("a root");
    assert!(verify(&named, DEPTH, Kind::Kernel, &ctx, &t1));
}

#[test]
fn another_kind_another_context_or_a_tampered_path_name_another_root() {
    let (root, tree, _, kernel) = policy();
    let t1 = tree.trailer(1).expect("trailer 1");
    let ctx = kernel_ctx(&kernel, EPOCH);
    assert_ne!(root_of(DEPTH, Kind::Bootloader, &ctx, &t1), Some(root));
    assert_ne!(root_of(DEPTH, Kind::Kernel, &kernel_ctx(&kernel, EPOCH + 1), &t1), Some(root));
    for at in 9..t1.len() {
        let mut t = t1.clone();
        t[at] ^= 1;
        assert_ne!(root_of(DEPTH, Kind::Kernel, &ctx, &t), Some(root), "byte {at}");
    }
}

#[test]
fn the_pad_kind_a_wrong_depth_or_a_cut_path_name_no_root() {
    let (_, tree, _, kernel) = policy();
    let t1 = tree.trailer(1).expect("trailer 1");
    let ctx = kernel_ctx(&kernel, EPOCH);
    assert_eq!(root_of(DEPTH, Kind::Pad, &ctx, &t1), None);
    assert_eq!(root_of(DEPTH + 1, Kind::Kernel, &ctx, &t1), None);
    for cut in 0..t1.len() {
        assert_eq!(root_of(DEPTH, Kind::Kernel, &ctx, &t1[..cut]), None, "cut at {cut}");
    }
}
