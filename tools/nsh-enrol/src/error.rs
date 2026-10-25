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

//! Why the tool refused. Every refusal names what was wrong, so an
//! operator never signs a certificate the client would then refuse.

use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// The document failed one of the client's own checks.
    Doc(nsh_attest::Refusal),
    /// The document was made for another challenge than the one given.
    Challenge,
    /// The document's AK is not the one the registrar activated.
    Ak,
    /// Replaying the log does not give the quoted PCR.
    Replay(u8),
    /// No release loader ran: the machine is not running a release.
    NoLoader,
    /// More boot applications than a certificate carries.
    BootApplications(usize),
    /// An application in PCR 4 the operator did not expect.
    Unexpected([u8; 32]),
    /// A host name, fleet id, validity or other value is not valid.
    Value(&'static str),
    /// A key file is not one this tool wrote.
    KeyFile,
    /// The randomness could not be drawn, or signing failed.
    Randomness,
    /// The certificate made does not verify; nothing is written.
    SelfCheck,
    Io(String),
    Usage(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Doc(r) => write!(f, "the attestation document is refused: {r}"),
            Error::Challenge => f.write_str("the document answers another challenge"),
            Error::Ak => f.write_str("the document's AK is not the activated one"),
            Error::Replay(i) => write!(f, "the log does not replay to PCR {i}"),
            Error::NoLoader => f.write_str("no release loader is in PCR 4"),
            Error::BootApplications(n) => write!(f, "{n} boot applications, at most 8"),
            Error::Unexpected(a) => write!(f, "unexpected boot application {}", crate::hex::of(a)),
            Error::Value(what) => write!(f, "invalid {what}"),
            Error::KeyFile => f.write_str("not an nsh-enrol key file"),
            Error::Randomness => f.write_str("no randomness, or signing failed"),
            Error::SelfCheck => f.write_str("the certificate made does not verify"),
            Error::Io(e) => write!(f, "{e}"),
            Error::Usage(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for Error {}
