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

//! The exchange's state, its start and its own commit.

use super::super::commit::{Commit, SaeKeys};
use super::super::frame::TOKEN_MAX;
use super::super::group::LEN;
use super::types::{SaeFailure, SaeState};
use p256::ProjectivePoint;

#[derive(Clone)]
pub struct SaeStation {
    pub(super) state: SaeState,
    pub(super) h2e: bool,
    pub(super) commit: Commit,
    pub(super) own_scalar: [u8; LEN],
    pub(super) own_element: [u8; 2 * LEN],
    pub(super) token: [u8; TOKEN_MAX],
    pub(super) token_len: usize,
    pub(super) keys: SaeKeys,
    pub(super) peer_scalar: [u8; LEN],
    pub(super) peer_element: [u8; 2 * LEN],
    pub(super) failure: Option<SaeFailure>,
}

impl SaeStation {
    /// Start an exchange from the PWE and two fresh 32-byte random values.
    /// `None` if they do not make a valid commit (draw again).
    pub fn start(pwe: ProjectivePoint, h2e: bool, rand: &[u8; LEN], mask: &[u8; LEN]) -> Option<Self> {
        let commit = Commit::new(pwe, rand, mask)?;
        let (own_scalar, own_element) = (commit.scalar(), commit.element()?);
        Some(SaeStation {
            state: SaeState::Committed,
            h2e,
            commit,
            own_scalar,
            own_element,
            token: [0u8; TOKEN_MAX],
            token_len: 0,
            keys: SaeKeys { kck: [0u8; 32], pmk: [0u8; 32], pmkid: [0u8; 16] },
            peer_scalar: [0u8; LEN],
            peer_element: [0u8; 2 * LEN],
            failure: None,
        })
    }

    pub fn state(&self) -> SaeState {
        self.state
    }

    /// Why the last frame was discarded, while the exchange runs.
    pub fn failure(&self) -> Option<SaeFailure> {
        self.failure
    }

    /// The PMK and PMKID once accepted.
    pub fn pmk(&self) -> Option<([u8; 32], [u8; 16])> {
        (self.state == SaeState::Accepted).then_some((self.keys.pmk, self.keys.pmkid))
    }
}
