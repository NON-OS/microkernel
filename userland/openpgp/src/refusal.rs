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

//! Why a signature was not accepted, each named so a log says which.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Refusal {
    Malformed,
    Version,
    NotBinary,
    WeakHash,
    UnknownHash,
    Algorithm,
    Critical,
    UnknownKey,
    QuickCheck,
    BadSignature,
}

impl Refusal {
    pub fn why(self) -> &'static str {
        match self {
            Refusal::Malformed => "the signature does not parse",
            Refusal::Version => "not a version 4 signature",
            Refusal::NotBinary => "not a signature over a binary document",
            Refusal::WeakHash => "made with MD5 or SHA-1, which are refused",
            Refusal::UnknownHash => "a digest other than SHA-256 or SHA-512",
            Refusal::Algorithm => "an algorithm the key does not have",
            Refusal::Critical => "a critical subpacket this verifier does not know",
            Refusal::UnknownKey => "made by no key in the pinned keyring",
            Refusal::QuickCheck => "the digest does not match the signature's check bytes",
            Refusal::BadSignature => "the signature does not verify",
        }
    }
}
