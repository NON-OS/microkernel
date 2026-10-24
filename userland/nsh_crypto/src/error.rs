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

//! Why a cryptographic step failed. None of these is told to the peer in
//! more detail than the wire's reason codes allow.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CryptoError {
    /// The peer's X-Wing encapsulation key is not a valid key.
    BadKey,
    /// A sealed frame did not open: wrong key, wrong order, or changed.
    Open,
    /// A direction has sealed its last frame or its last byte.
    Limit,
    /// An output buffer cannot hold the result.
    Room,
}
