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

//! `release.kernel-signature`: the kernel the seal signed verifies under the
//! tree's kernel keys in both halves; a change to its bytes, to either
//! signature, to its footer or to the keys fails it, and says what.

use super::attest::KERNEL;
use super::fake::Fake;
use super::sealed_fixture::Sealed;
use crate::check::{Check, Status};
use crate::release::{kernel_check, load};
use std::fs;

pub fn kernel_of(s: &Sealed) -> Check {
    let mut ctx = s.ctx(Fake::default());
    load::check(&mut ctx);
    kernel_check::check(&mut ctx)
}

/// One byte of the kernel changed, on the ESP too, and the record rewritten to
/// match, as one who holds the release can: only the signature can catch it.
pub fn poke(s: &Sealed, at: impl Fn(usize) -> usize) {
    let mut b = fs::read(s.release.path().join("kernel.bin")).unwrap();
    let i = at(b.len());
    b[i] ^= 0x01;
    s.release.put("kernel.bin", &b);
    s.release.put("esp/kernel.bin", &b);
    s.release.record();
}

pub fn fails_with(s: &Sealed, why: &str) {
    let c = kernel_of(s);
    assert_eq!(c.status, Status::Fail, "{}", c.reason);
    assert!(c.reason.contains(why), "{why:?} not in {:?}", c.reason);
}

#[test]
fn the_kernel_as_sealed_verifies_in_both_halves() {
    let c = kernel_of(&Sealed::dev());
    assert_eq!(c.status, Status::Pass, "{}", c.reason);
    assert!(c.reason.contains("Ed25519 and ML-DSA-65 verify"));
}

#[test]
fn a_changed_byte_of_the_kernel_breaks_its_signature() {
    for at in [0, KERNEL.len() / 2, KERNEL.len() - 1] {
        let s = Sealed::dev();
        poke(&s, |_| at);
        fails_with(&s, "Ed25519 signature does not verify");
    }
}

#[test]
fn a_changed_byte_of_either_signature_alone_is_caught() {
    let s = Sealed::dev();
    poke(&s, |_| KERNEL.len() + 40 + 5);
    fails_with(&s, "Ed25519 signature does not verify");
    let s = Sealed::dev();
    poke(&s, |_| KERNEL.len() + 136 + 5);
    fails_with(&s, "ML-DSA-65 signature does not verify");
}
