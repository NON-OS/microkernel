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

//! Which releases the gate offers, and which it holds back.

use super::fixture::release;
use crate::install_ready::arch::{HOSTED_ARCH, RUNNING_ARCH};
use crate::install_ready::checks::evaluate;

/*
 * Only a machine that hosts a second triple has distribution packages. The
 * host arch decides which, so the test takes it from the gate rather than
 * naming x86_64.
 */
#[test]
fn a_distribution_package_is_ready_without_shipping_a_proof() {
    if HOSTED_ARCH.is_empty() {
        return;
    }
    assert!(evaluate(true, "linux.pkg", &release(HOSTED_ARCH, 0), true, false).install_ready);
}

#[test]
fn a_machine_that_hosts_nothing_readies_no_distribution_package() {
    if !HOSTED_ARCH.is_empty() {
        return;
    }
    assert!(!evaluate(true, "linux.pkg", &release("x86_64-linux", 0), true, false).install_ready);
}

#[test]
fn naming_the_hosted_arch_does_not_exempt_a_capsule_from_its_proof() {
    let r = evaluate(true, "nonos.app.pkg", &release("x86_64-linux", 0), true, false);
    assert!(!r.install_ready && !r.attestation_present);
}

#[test]
fn a_capsule_that_ships_a_proof_is_ready() {
    assert!(evaluate(true, "nonos.app.pkg", &release(RUNNING_ARCH, 9), true, false).install_ready);
}

#[test]
fn no_signature_no_readiness() {
    let rel = release("x86_64-linux", 0);
    assert!(!evaluate(false, "linux.pkg", &rel, true, false).install_ready);
    assert!(!evaluate(true, "linux.pkg", &rel, false, false).install_ready);
}
