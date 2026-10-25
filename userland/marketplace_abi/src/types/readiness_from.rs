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

//! The verdict made: refused outright, or composed from the six checks.

use super::InstallReadiness;

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
