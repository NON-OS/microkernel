// NONOS Operating System (AGPL-3.0-or-later)

//! A commit from the access point: its own, a token request, or a refusal.

use super::super::frame::{
    parse_token_request, STATUS_ANTI_CLOGGING_TOKEN_REQ, STATUS_GROUP_NOT_SUPPORTED,
    STATUS_SAE_HASH_TO_ELEMENT, STATUS_SUCCESS,
};
use super::{SaeFailure, SaeState, SaeStation, SaeStep};

/// Token requests answered in one exchange: dot11RSNASAESync's default. A
/// forger who keeps asking costs this many commits, not one per request until
/// the deadline.
pub const TOKEN_ROUNDS_MAX: u8 = 5;

impl SaeStation {
    pub(super) fn on_commit(&mut self, status: u16, body: &[u8]) -> SaeStep {
        match status {
            STATUS_ANTI_CLOGGING_TOKEN_REQ if self.state == SaeState::Committed => {
                if self.token_rounds >= TOKEN_ROUNDS_MAX {
                    return self.discard(SaeFailure::TooManyTokenRequests);
                }
                let Some(token) = parse_token_request(body, self.h2e) else {
                    return self.discard(SaeFailure::BadCommit);
                };
                self.token_rounds += 1;
                self.token[..token.len()].copy_from_slice(token);
                self.token_len = token.len();
                let (seq, status, body) = self.commit_frame();
                SaeStep::Send { seq, status, body }
            }
            STATUS_GROUP_NOT_SUPPORTED => self.discard(SaeFailure::GroupNotSupported),
            STATUS_SUCCESS | STATUS_SAE_HASH_TO_ELEMENT => {
                if self.state != SaeState::Committed {
                    return SaeStep::Ignore;
                }
                if (status == STATUS_SAE_HASH_TO_ELEMENT) != self.h2e {
                    return self.discard(SaeFailure::BadCommit);
                }
                self.accept_commit(body)
            }
            other => self.discard(SaeFailure::Rejected(other)),
        }
    }
}
