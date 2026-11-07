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

//! The outbox sent over a channel that may go down mid-way, and what a lost
//! channel does to the model: a command the channel went down on is kept,
//! with every one after it, and a channel up again is told the network
//! before anything else, as a new core must be, then asked for the chats.

use alloc::string::String;
use alloc::vec::Vec;
use core::mem::take;

use super::focus::Conn;
use super::state::Model;
use crate::proto::cmd::Cmd;

/// The notice while the core is started again.
pub const LOST: &str = "the channel to the core closed; starting it again";

impl Model {
    /// Each command in the outbox, oldest first, to `send`, which answers
    /// whether the channel is still up after it. The first it answers false
    /// for is kept, with all after it; true when every command went.
    pub fn flush(&mut self, mut send: impl FnMut(&Cmd) -> bool) -> bool {
        let mut cmds = take(&mut self.outbox).into_iter();
        for cmd in cmds.by_ref() {
            if !send(&cmd) {
                self.outbox = core::iter::once(cmd).chain(cmds).collect();
                return false;
            }
        }
        true
    }

    /// The channel went down after it was up. The window says so, and the
    /// commands not yet sent wait behind the network and the chat list.
    pub fn lost(&mut self) {
        self.conn = Conn::Offline;
        self.notice = String::from(LOST);
        let kept: Vec<Cmd> = take(&mut self.outbox)
            .into_iter()
            .filter(|c| !matches!(c, Cmd::Network | Cmd::Chats))
            .collect();
        self.outbox = [Cmd::Network, Cmd::Chats].into_iter().chain(kept).collect();
    }
}
