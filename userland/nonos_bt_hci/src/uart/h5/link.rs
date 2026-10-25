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

//! Link establishment (Vol 4, Part D, "Link Establishment"). Each side
//! starts Uninitialized and sends SYNC until it hears SYNC RESPONSE; it is
//! then Initialized and sends CONFIG until it hears CONFIG RESPONSE; it is
//! then Active. In every state it answers SYNC with SYNC RESPONSE and,
//! once past Uninitialized, CONFIG with CONFIG RESPONSE.
//!
//! A SYNC heard while Active means the peer has started over, and this
//! side's numbering belongs to a link that no longer exists.

use alloc::vec::Vec;

use super::config::{Agreed, OURS};
use super::control::{with_field, CONFIG, SYNC};

/// How often SYNC and CONFIG are sent until answered, in milliseconds.
pub const RESEND_MS: u64 = 250;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum State {
    Uninitialized,
    Initialized,
    Active,
}

pub struct Link {
    pub state: State,
    pub agreed: Agreed,
    pub(super) last: Option<u64>,
}

impl Default for Link {
    fn default() -> Self {
        Self::new()
    }
}

impl Link {
    pub const fn new() -> Self {
        Self {
            state: State::Uninitialized,
            agreed: Agreed { window: 1, oof: false, crc: false },
            last: None,
        }
    }

    /// SYNC or CONFIG, when one is due at `now`.
    pub fn due(&mut self, now: u64) -> Option<Vec<u8>> {
        let msg = match self.state {
            State::Uninitialized => SYNC.to_vec(),
            State::Initialized => with_field(CONFIG, OURS.field()),
            State::Active => return None,
        };
        let ready = self.last.is_none_or(|t| now.saturating_sub(t) >= RESEND_MS);
        ready.then(|| {
            self.last = Some(now);
            msg
        })
    }
}
