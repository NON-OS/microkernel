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

//! Each refusal in plain words, as the window shows it.

use nonos_device_attest::RegistryError;

use super::error::Refusal;

impl Refusal {
    pub fn why(&self) -> &'static str {
        match self {
            Refusal::RequestSize => "the request is not the length its fields give",
            Refusal::RequestMagic => "the request file is not a proof request",
            Refusal::RequestNonceWord => "the request's nonce is not four field words",
            Refusal::RequestRootWord => "the request's registry root is not four field words",
            Refusal::RequestVerifierLength => "the request names no verifier, or too long a one",
            Refusal::RequestVerifierByte => "the verifier's name has a byte that cannot be shown",
            Refusal::TranscriptSize => "the registry transcript is larger than any registry",
            Refusal::TranscriptText => "the registry transcript is not text",
            Refusal::Transcript(e) => transcript(e),
            Refusal::RootNotRequested => "the registry is not the one the verifier accepts",
            Refusal::NoEk => "the TPM gave no endorsement key",
            Refusal::EkAnswer => "the TPM's endorsement key answer is malformed",
            Refusal::NotEnrolled => "this device is not enrolled in the registry",
            Refusal::CommitmentMismatch => "the registry holds another secret for this device",
            Refusal::SecretWord => "the device secret is not four field words",
            Refusal::Slots => "the kernel's boot slots record does not parse",
            Refusal::Scope => "the verifier's scope cannot be derived",
            Refusal::OutputSize => "the proof is larger than the output file holds",
        }
    }
}

fn transcript(e: &RegistryError) -> &'static str {
    match e {
        RegistryError::Depth => "the registry's depth is not one the proof takes",
        RegistryError::Full => "the registry lists more devices than its depth holds",
        RegistryError::Revoked => "the registry enrolls a revoked key",
        RegistryError::Duplicate => "the registry holds one commitment under two keys",
        RegistryError::Transcript(_) => "the registry transcript does not give its own root",
    }
}
