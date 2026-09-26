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

use nonos_marketplace_abi::{CapsuleRelease, InstallReadiness, ValidationStatus};

use super::arch::{HOSTED_ARCH, RUNNING_ARCH};

pub const RUNNING_KERNEL_ABI: u32 = 1;

/// Listings under this namespace are distribution packages. The store sends
/// them to the Linux installer, which authenticates the bytes against the
/// distribution's own signatures and has the machine mint their proof.
const HOSTED_NAMESPACE: &str = "linux.";

pub fn evaluate(
    signature_verified: bool,
    listing_id: &str,
    release: &CapsuleRelease,
    publisher_signature_verified: bool,
) -> InstallReadiness {
    let index_signature_valid = signature_verified;
    let validation_passed = release.validation.status == ValidationStatus::Validated;
    let package_url_present = !release.package_url.is_empty();
    let package_hash_present = release.package_hash.iter().any(|&b| b != 0);
    let manifest_hash_present = release.manifest_hash.iter().any(|&b| b != 0);
    let runs_here = |a: &alloc::string::String| {
        a.as_str() == RUNNING_ARCH || (!HOSTED_ARCH.is_empty() && a.as_str() == HOSTED_ARCH)
    };
    let arch_match = release.supported_arches.iter().any(runs_here);
    let kernel_abi_compatible = release.kernel_abi_min <= RUNNING_KERNEL_ABI;
    /*
     * Exempting a release from shipping a proof because it names an arch let
     * any release exempt itself. The exemption now follows the namespace the
     * store routes on, so a release earns it only by going where the proof
     * is minted after its bytes are authenticated.
     */
    let minted_locally = listing_id.starts_with(HOSTED_NAMESPACE)
        && release.supported_arches.iter().any(|a| a.as_str() == HOSTED_ARCH);
    let ships_proof = release.zk_trailer_hash.iter().any(|&b| b != 0);
    let attestation_present = ships_proof || minted_locally;

    let install_ready = index_signature_valid
        && validation_passed
        && package_url_present
        && package_hash_present
        && manifest_hash_present
        && publisher_signature_verified
        && arch_match
        && kernel_abi_compatible
        && attestation_present;

    InstallReadiness {
        install_ready,
        index_signature_valid,
        package_url_present: package_url_present && package_hash_present && manifest_hash_present,
        publisher_signature_present: publisher_signature_verified,
        validation_passed,
        arch_match: arch_match && kernel_abi_compatible,
        attestation_present,
    }
}
