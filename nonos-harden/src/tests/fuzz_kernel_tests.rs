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

//! The genuine signed and attested kernel, mutated and cut: its parser never
//! panics, and no mutation makes the signature check accept a kernel other
//! than the one signed, or a rollback index other than the one signed.

use super::attest::{KERNEL, ROLLBACK};
use super::fuzz::{genuine, mutated, runner};
use crate::release::kernel::parse;
use crate::release::kernel_check::verify;

#[test]
fn the_genuine_kernel_verifies() {
    let g = genuine();
    assert!(verify(&g.kernel, g.sealed.tree.path(), Some(ROLLBACK)).is_ok());
}

#[test]
fn no_mutation_is_accepted_as_another_kernel() {
    let g = genuine();
    runner(512)
        .run(&mutated(&g.kernel, 4), |b| {
            if verify(&b, g.sealed.tree.path(), None).is_ok() {
                let a = parse(&b).unwrap();
                assert_eq!((a.kernel, a.rollback), (KERNEL, ROLLBACK));
            }
            Ok(())
        })
        .unwrap();
}

#[test]
fn every_cut_of_the_kernel_is_refused() {
    let g = genuine();
    for len in 0..g.kernel.len() {
        assert!(parse(&g.kernel[..len]).is_err(), "cut to {len} was accepted");
    }
}
