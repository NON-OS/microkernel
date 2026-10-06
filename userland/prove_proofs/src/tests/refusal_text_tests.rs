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

//! Every refusal reads differently in the window, in plain ASCII.

use std::collections::BTreeSet;

use nonos_device_attest::RegistryError;

use crate::assemble::error::Refusal;

#[test]
fn every_refusal_has_words() {
    let all = [
        Refusal::RequestSize,
        Refusal::RequestMagic,
        Refusal::RequestNonceWord,
        Refusal::RequestRootWord,
        Refusal::RequestVerifierLength,
        Refusal::RequestVerifierByte,
        Refusal::TranscriptSize,
        Refusal::TranscriptText,
        Refusal::Transcript(RegistryError::Depth),
        Refusal::Transcript(RegistryError::Full),
        Refusal::Transcript(RegistryError::Revoked),
        Refusal::Transcript(RegistryError::Duplicate),
        Refusal::Transcript(RegistryError::Transcript("any".into())),
        Refusal::RootNotRequested,
        Refusal::NoEk,
        Refusal::EkAnswer,
        Refusal::NotEnrolled,
        Refusal::CommitmentMismatch,
        Refusal::SecretWord,
        Refusal::Slots,
        Refusal::Scope,
        Refusal::OutputSize,
    ];
    let words: BTreeSet<_> = all.iter().map(Refusal::why).collect();
    assert_eq!(words.len(), all.len(), "two refusals read the same");
    assert!(words.iter().all(|w| !w.is_empty() && w.is_ascii()));
}
