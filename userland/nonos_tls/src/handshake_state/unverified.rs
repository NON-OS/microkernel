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

//! Why a complete flight did not verify.

use crate::crypto_port::PoolSilent;
use crate::server_complete::ServerComplete;

/// A flight that did not verify: a check failed, or the crypto pool never
/// answered one (CRYPTO_WAIT_MS). Both refuse; only the reason differs.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Unverified {
    /// Incomplete, malformed, or a chain, signature or MAC that is wrong.
    Failed,
    /// A signature check the pool took and did not answer in time.
    TooSlow,
}

impl From<PoolSilent> for Unverified {
    fn from(_: PoolSilent) -> Self {
        Unverified::TooSlow
    }
}

/// What `HandshakeState::verify` gives back.
pub type Verified = Result<ServerComplete, Unverified>;
