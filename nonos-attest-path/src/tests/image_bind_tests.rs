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

/*
 * T2 at the gate the kernel runs: an image is admitted only if its own
 * measurement was enrolled. The forger holds every enrolled path and presents
 * one for an image that was never enrolled, or for an enrolled image changed
 * by one byte; the leaf is built from the image's own measurement, so neither
 * folds to the root. stark_proofs keeps the private-leaf gate's counterexample.
 */

use super::fixture::{capsule_ctx, kernel_ctx, policy, DEPTH, EPOCH};
use crate::leaf::Kind;
use crate::verify::verify;

const ROGUE: &[u8] = b"\x7fELF never enrolled";

#[test]
fn no_enrolled_path_admits_an_image_never_enrolled() {
    let (root, tree, _, _) = policy();
    for slot in 0..(1 << DEPTH) {
        let t = tree.trailer(slot).expect("trailer");
        assert!(!verify(&root, DEPTH, Kind::Capsule, &capsule_ctx(ROGUE, 0x7, EPOCH), &t));
        assert!(!verify(&root, DEPTH, Kind::Kernel, &kernel_ctx(ROGUE, EPOCH), &t));
    }
}

#[test]
fn one_changed_byte_of_an_enrolled_image_is_refused() {
    let (root, tree, cap, kernel) = policy();
    let (t0, t1) = (tree.trailer(0).expect("t0"), tree.trailer(1).expect("t1"));
    for i in 0..cap.len() {
        let mut c = cap.clone();
        c[i] ^= 1;
        assert!(!verify(&root, DEPTH, Kind::Capsule, &capsule_ctx(&c, 0x7, EPOCH), &t0));
    }
    for i in 0..kernel.len() {
        let mut k = kernel.clone();
        k[i] ^= 0x80;
        assert!(!verify(&root, DEPTH, Kind::Kernel, &kernel_ctx(&k, EPOCH), &t1));
    }
}
