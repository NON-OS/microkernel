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

//! Verdict the capsule emits when a caller asks "is this release ready to
//! install?".

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct InstallReadiness {
    /// Final answer: install allowed iff all five flags are true.
    pub install_ready: bool,
    /// `index_signature` verifies against the operator pubkey.
    pub index_signature_valid: bool,
    /// `package_url` is non-empty.
    pub package_url_present: bool,
    /// Publisher signature verifies against the listing pubkey.
    pub publisher_signature_present: bool,
    /// Operator's `validation_status` is `Validated`.
    pub validation_passed: bool,
    /// Running kernel arch is in the release's `supported_arches`.
    pub arch_match: bool,
    /// The release names a zk trailer binding its own measurement to the
    /// enrolled set.
    pub attestation_present: bool,
}

impl InstallReadiness {
    pub fn refused() -> Self {
        Self {
            install_ready: false,
            index_signature_valid: false,
            package_url_present: false,
            publisher_signature_present: false,
            validation_passed: false,
            arch_match: false,
            attestation_present: false,
        }
    }

    /// Compose a verdict from the six checks. `install_ready` is
    /// the AND of the inputs; anything `false` blocks install.
    pub fn from_checks(
        index_signature_valid: bool,
        package_url_present: bool,
        publisher_signature_verified: bool,
        validation_passed: bool,
        arch_match: bool,
        attestation_present: bool,
    ) -> Self {
        let install_ready = index_signature_valid
            && package_url_present
            && publisher_signature_verified
            && validation_passed
            && arch_match
            && attestation_present;
        Self {
            install_ready,
            index_signature_valid,
            package_url_present,
            publisher_signature_present: publisher_signature_verified,
            validation_passed,
            arch_match,
            attestation_present,
        }
    }
}
