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

//! The station side of one SAE exchange with an access point (IEEE Std
//! 802.11-2020, 12.4.8): send a commit; answer an anti-clogging request by
//! sending it again with the token; on the AP's commit derive the keys and
//! send a confirm; on the AP's confirm check it and accept. The status code of
//! the AP's commit must say the same PWE method this station used (126 for
//! hash-to-element, 0 for hunting and pecking), a commit that only reflects
//! this station's own is dropped, and anything malformed or refused ends the
//! exchange with the reason kept for the panel.

use alloc::vec::Vec;

use p256::ProjectivePoint;

use super::commit::{confirm, Commit, SaeKeys};
use super::frame::{
    commit_body, confirm_body, parse_commit, parse_confirm, parse_token_request,
    STATUS_ANTI_CLOGGING_TOKEN_REQ, STATUS_GROUP_NOT_SUPPORTED, STATUS_SAE_HASH_TO_ELEMENT,
    STATUS_SUCCESS, SEQ_COMMIT, SEQ_CONFIRM, TOKEN_MAX,
};
use super::group::LEN;

/// Where the exchange stands.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SaeState {
    Committed,
    Confirmed,
    Accepted,
    Failed,
}

/// Why an exchange ended without a PMK.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SaeFailure {
    /// The AP answered with this status code (a wrong password ends here
    /// only at the confirm: the AP cannot tell before).
    Rejected(u16),
    /// The AP does not run group 19.
    GroupNotSupported,
    /// The AP's commit was malformed, used the other PWE method, or gave an
    /// identity shared secret.
    BadCommit,
    /// The AP's confirm did not verify: a different password.
    BadConfirm,
}

/// What a received SAE frame calls for.
pub enum SaeStep {
    /// Send an Authentication frame with this transaction sequence, status
    /// code and SAE body.
    Send { seq: u16, status: u16, body: Vec<u8> },
    /// The AP's confirm verified: the PMK is ready.
    Accepted,
    /// Nothing to do (a duplicate, or a frame for another state).
    Ignore,
    /// The exchange failed; see `SaeStation::failure`.
    Failed,
}

#[derive(Clone, Copy)]
pub struct SaeStation {
    state: SaeState,
    h2e: bool,
    commit: Commit,
    own_scalar: [u8; LEN],
    own_element: [u8; 2 * LEN],
    token: [u8; TOKEN_MAX],
    token_len: usize,
    keys: SaeKeys,
    peer_scalar: [u8; LEN],
    peer_element: [u8; 2 * LEN],
    failure: Option<SaeFailure>,
}

impl SaeStation {
    /// Start an exchange from the PWE and two fresh 32-byte random values.
    /// `None` if they do not make a valid commit (draw again).
    pub fn start(pwe: ProjectivePoint, h2e: bool, rand: &[u8; LEN], mask: &[u8; LEN]) -> Option<Self> {
        let commit = Commit::new(pwe, rand, mask)?;
        Some(SaeStation {
            state: SaeState::Committed,
            h2e,
            commit,
            own_scalar: commit.scalar(),
            own_element: commit.element()?,
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

    pub fn failure(&self) -> Option<SaeFailure> {
        self.failure
    }

    /// The PMK and PMKID once accepted.
    pub fn pmk(&self) -> Option<([u8; 32], [u8; 16])> {
        (self.state == SaeState::Accepted).then_some((self.keys.pmk, self.keys.pmkid))
    }

    /// The commit to send: (transaction sequence, status code, body).
    pub fn commit_frame(&self) -> (u16, u16, Vec<u8>) {
        let status = if self.h2e { STATUS_SAE_HASH_TO_ELEMENT } else { STATUS_SUCCESS };
        let body = commit_body(&self.own_scalar, &self.own_element, &self.token[..self.token_len], self.h2e);
        (SEQ_COMMIT, status, body)
    }

    /// Feed an SAE Authentication frame from the AP.
    pub fn on_frame(&mut self, seq: u16, status: u16, body: &[u8]) -> SaeStep {
        if matches!(self.state, SaeState::Accepted | SaeState::Failed) {
            return SaeStep::Ignore;
        }
        match seq {
            SEQ_COMMIT => self.on_commit(status, body),
            SEQ_CONFIRM => self.on_confirm(status, body),
            _ => SaeStep::Ignore,
        }
    }

    fn on_commit(&mut self, status: u16, body: &[u8]) -> SaeStep {
        match status {
            STATUS_ANTI_CLOGGING_TOKEN_REQ if self.state == SaeState::Committed => {
                let Some(token) = parse_token_request(body, self.h2e) else {
                    return self.fail(SaeFailure::BadCommit);
                };
                self.token[..token.len()].copy_from_slice(token);
                self.token_len = token.len();
                let (seq, status, body) = self.commit_frame();
                SaeStep::Send { seq, status, body }
            }
            STATUS_GROUP_NOT_SUPPORTED => self.fail(SaeFailure::GroupNotSupported),
            STATUS_SUCCESS | STATUS_SAE_HASH_TO_ELEMENT => {
                if self.state != SaeState::Committed {
                    return SaeStep::Ignore;
                }
                if (status == STATUS_SAE_HASH_TO_ELEMENT) != self.h2e {
                    return self.fail(SaeFailure::BadCommit);
                }
                self.accept_commit(body)
            }
            other => self.fail(SaeFailure::Rejected(other)),
        }
    }

    fn accept_commit(&mut self, body: &[u8]) -> SaeStep {
        let Some(peer) = parse_commit(body) else {
            return self.fail(SaeFailure::BadCommit);
        };
        // A commit equal to this station's own is a reflection: drop it silently.
        if peer.scalar_bytes == self.own_scalar && peer.element_bytes == self.own_element {
            return SaeStep::Ignore;
        }
        let Some(keys) = self.commit.derive_keys(&peer.scalar, &peer.element) else {
            return self.fail(SaeFailure::BadCommit);
        };
        self.keys = keys;
        self.peer_scalar = peer.scalar_bytes;
        self.peer_element = peer.element_bytes;
        self.state = SaeState::Confirmed;
        // The first confirm this station sends carries send-confirm 1.
        let send_confirm = 1u16;
        let c = confirm(
            &self.keys.kck,
            send_confirm,
            &self.own_scalar,
            &self.own_element,
            &self.peer_scalar,
            &self.peer_element,
        );
        SaeStep::Send { seq: SEQ_CONFIRM, status: STATUS_SUCCESS, body: confirm_body(send_confirm, &c) }
    }

    fn on_confirm(&mut self, status: u16, body: &[u8]) -> SaeStep {
        if status != STATUS_SUCCESS {
            return self.fail(SaeFailure::Rejected(status));
        }
        if self.state != SaeState::Confirmed {
            return SaeStep::Ignore;
        }
        let Some((peer_sc, got)) = parse_confirm(body) else {
            return self.fail(SaeFailure::BadConfirm);
        };
        let want = confirm(
            &self.keys.kck,
            peer_sc,
            &self.peer_scalar,
            &self.peer_element,
            &self.own_scalar,
            &self.own_element,
        );
        let mut diff = 0u8;
        for (a, b) in got.iter().zip(want.iter()) {
            diff |= a ^ b;
        }
        if diff != 0 {
            return self.fail(SaeFailure::BadConfirm);
        }
        self.state = SaeState::Accepted;
        SaeStep::Accepted
    }

    fn fail(&mut self, why: SaeFailure) -> SaeStep {
        self.state = SaeState::Failed;
        self.failure = Some(why);
        SaeStep::Failed
    }
}
