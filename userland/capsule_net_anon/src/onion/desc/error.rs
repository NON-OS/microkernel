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


//! Why a descriptor was not used.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DescError {
    /// A field missing, repeated where it may not be, or out of range.
    Malformed,
    /// A certificate of the wrong type, signer or age, or a bad signature.
    Certificate,
    /// Certified by a blinded key other than the one this address gives.
    WrongService,
    /// The descriptor's own signature does not verify.
    Signature,
    /// A layer's MAC failed or it decrypted to nothing.
    Layer,
    /// The inner layer only opens with a client authorization key, which
    /// this client does not hold.
    ClientAuth,
    /// The service lists no introduction point this client can reach.
    NoIntroPoints,
}

impl DescError {
    /// The log line for a descriptor refused this way.
    pub fn said(self) -> &'static [u8] {
        match self {
            DescError::Malformed => b"onion descriptor refused, malformed",
            DescError::Certificate => b"onion descriptor refused, certificate",
            DescError::WrongService => b"onion descriptor refused, another service's",
            DescError::Signature => b"onion descriptor refused, signature",
            DescError::Layer => b"onion descriptor refused, layer did not open",
            DescError::ClientAuth => b"onion service needs client authorization",
            DescError::NoIntroPoints => b"onion descriptor has no usable introduction point",
        }
    }
}
