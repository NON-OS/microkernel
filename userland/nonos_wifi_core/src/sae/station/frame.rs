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

//! Feed an SAE Authentication frame from the AP.

use super::*;

impl SaeStation {
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

    pub(super) fn on_commit(&mut self, status: u16, body: &[u8]) -> SaeStep {
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
}
