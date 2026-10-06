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

//! Why a circuit could not be built.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BuildError {
    Link,
    Protocol,
    Destroyed,
    Timeout,
    Handshake,
    Unrecognised,
}

impl BuildError {
    /// The log line for a build that failed this way, so the cause is named.
    pub fn said(self) -> &'static [u8] {
        match self {
            BuildError::Link => b"circuit build failed, link write",
            BuildError::Protocol => b"circuit build failed, unexpected reply",
            BuildError::Destroyed => b"circuit build failed, destroyed by a relay",
            BuildError::Timeout => b"circuit build failed, no answer in time",
            BuildError::Handshake => b"circuit build failed, ntor handshake",
            BuildError::Unrecognised => b"circuit build failed, reply matched no hop",
        }
    }
}
