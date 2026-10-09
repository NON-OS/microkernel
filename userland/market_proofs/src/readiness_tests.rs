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

use alloc::string::String;
use alloc::vec;

use nonos_marketplace_abi::{CapsuleRelease, ValidationReport, ValidationStatus};

use crate::install_ready::arch::{HOSTED_ARCH, RUNNING_ARCH};
use crate::install_ready::checks::evaluate;

pub(crate) fn release(arch: &str, trailer: u8) -> CapsuleRelease {
    CapsuleRelease {
        release_id: String::from("pkg@1"),
        manifest_hash: [1; 32],
        package_hash: [2; 32],
        package_url: String::from("http://mirror/pkg-1.apk"),
        publisher_signature: vec![0; 64],
        supported_arches: vec![String::from(arch)],
        kernel_abi_min: 1,
        required_capabilities: vec![],
        zk_trailer_hash: [trailer; 32],
        validation: ValidationReport {
            status: ValidationStatus::Validated,
            note: String::new(),
            validator_id: String::from("v"),
            validated_at_ms: 1,
        },
    }
}

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
    assert!(evaluate(true, "linux.pkg", &release(HOSTED_ARCH, 0), true).install_ready);
}

#[test]
fn a_machine_that_hosts_nothing_readies_no_distribution_package() {
    if !HOSTED_ARCH.is_empty() {
        return;
    }
    assert!(!evaluate(true, "linux.pkg", &release("x86_64-linux", 0), true).install_ready);
}

#[test]
fn naming_the_hosted_arch_does_not_exempt_a_capsule_from_its_proof() {
    let r = evaluate(true, "nonos.app.pkg", &release("x86_64-linux", 0), true);
    assert!(!r.install_ready && !r.attestation_present);
}

#[test]
fn a_capsule_that_ships_a_proof_is_ready() {
    assert!(evaluate(true, "nonos.app.pkg", &release(RUNNING_ARCH, 9), true).install_ready);
}

#[test]
fn no_signature_no_readiness() {
    let rel = release("x86_64-linux", 0);
    assert!(!evaluate(false, "linux.pkg", &rel, true).install_ready);
    assert!(!evaluate(true, "linux.pkg", &rel, false).install_ready);
}
