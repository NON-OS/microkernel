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

//! The keys typed since the last turn, sent; Ctrl-] or the end of input
//! closes the session.

use super::session::Nsh;
use super::term::CLOSE_KEY;
use crate::Refusal;
use alloc::vec::Vec;
use std::sync::mpsc::{Receiver, TryRecvError};

/// Send the keys typed since the last turn; whether to keep reading them.
pub(super) fn keys_in(c: &mut Nsh, keys: &Receiver<Vec<u8>>) -> Result<bool, Refusal> {
    loop {
        let typed = match keys.try_recv() {
            Ok(k) => k,
            Err(TryRecvError::Empty) => return Ok(true),
            Err(TryRecvError::Disconnected) => Vec::new(),
        };
        let at = typed.iter().position(|b| *b == CLOSE_KEY);
        let send = &typed[..at.unwrap_or(typed.len())];
        // Once the shell has exited the session takes no more keys, and
        // what is left is the host's record and CLOSE.
        let sent = match send.is_empty() {
            true => Ok(()),
            false => c.input(send),
        };
        match sent {
            Err(Refusal::NotNow) => return Ok(false),
            Err(e) => return Err(e),
            Ok(()) => {}
        }
        if typed.is_empty() || at.is_some() {
            return match c.close() {
                Ok(()) | Err(Refusal::NotNow) => Ok(false),
                Err(e) => Err(e),
            };
        }
    }
}
