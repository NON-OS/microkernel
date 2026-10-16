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

//! What each link control message calls for in each state (Vol 4, Part D,
//! "Link Establishment" and "Low Power").

use alloc::vec::Vec;

use super::config::OURS;
use super::control::{with_field, Message, CONFIG_RESPONSE, SYNC_RESPONSE, WOKEN};
use super::link::{Link, State};

/// What a heard message calls for.
#[derive(Debug, PartialEq, Eq)]
pub enum Heard {
    Nothing,
    Reply(Vec<u8>),
    /// The peer started over; so does this side.
    Restart,
}

impl Link {
    pub fn heard(&mut self, m: Message) -> Heard {
        match (self.state, m) {
            (State::Active, Message::Sync) => return Heard::Restart,
            (_, Message::Sync) => return Heard::Reply(SYNC_RESPONSE.to_vec()),
            (State::Uninitialized, Message::SyncResponse) => {
                (self.state, self.last) = (State::Initialized, None)
            }
            (State::Uninitialized, _) => {}
            (_, Message::Config(_)) => {
                return Heard::Reply(with_field(CONFIG_RESPONSE, OURS.field()))
            }
            (State::Initialized, Message::ConfigResponse(f)) => {
                (self.state, self.agreed) = (State::Active, OURS.with(f));
            }
            (State::Active, Message::Wakeup) => return Heard::Reply(WOKEN.to_vec()),
            _ => {}
        }
        Heard::Nothing
    }
}
