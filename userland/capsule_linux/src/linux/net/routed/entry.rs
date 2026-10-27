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

//! One stream this capsule carries for a guest, from its connect (or its
//! SOCKS request) to its close.

use alloc::vec::Vec;

use super::super::guest_route::Net;
use super::socks::Ver;

pub use super::phase::{Dest, Phase};

pub struct Entry {
    pub phase: Phase,
    pub dest: Dest,
    /// The SOCKS endpoint's conversation, once its request is heard: the
    /// version its replies are in.
    pub socks: Option<Ver>,
    /// What the guest reads before the stream: the endpoint's replies.
    pub out: Vec<u8>,
    /// When the open gives up (retry.rs).
    pub until: i64,
    /// The network last opened or waited on, and why it has not opened.
    pub why: (Net, &'static str),
}

impl Entry {
    pub fn new(phase: Phase, until: i64) -> Entry {
        let dest = Dest { host: Vec::new(), port: 0 };
        Entry { phase, dest, socks: None, out: Vec::new(), until, why: (Net::Unread, "") }
    }

    /// The SOCKS endpoint's conversation, at any stage.
    pub fn endpoint(&self) -> bool {
        self.socks.is_some() || matches!(self.phase, Phase::Asking(_) | Phase::Closed)
    }

    /// The named stream it holds at net.socks5, if any.
    pub fn stream(&self) -> Option<u32> {
        match &self.phase {
            Phase::Nym { stream, .. } => Some(*stream),
            Phase::Open(link) => Some(link.stream),
            _ => None,
        }
    }
}
