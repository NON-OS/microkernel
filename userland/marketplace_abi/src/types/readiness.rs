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

#[path = "readiness_from.rs"]
mod from;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct InstallReadiness {
    /// Final answer: install allowed iff all six flags are true.
    pub install_ready: bool,
    /// `index_signature` verifies against the operator pubkey.
    pub index_signature_valid: bool,
    /// `package_url` is non-empty and both hashes are non-zero.
    pub package_url_present: bool,
    /// Publisher signature verifies against the listing pubkey.
    pub publisher_signature_present: bool,
    /// Operator's `validation_status` is `Validated`.
    pub validation_passed: bool,
    /// A `supported_arches` entry runs here and `kernel_abi_min` is met.
    pub arch_match: bool,
    /// The release names a zk trailer, or it is a `linux.` listing whose
    /// proof the machine mints after install.
    pub attestation_present: bool,
}
