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

//! Input events are believed from the input router alone. Anything can send
//! the shell a frame shaped like a delivery; a window that did could type
//! into a prompt, or forge the secure attention chord. The kernel stamps the
//! sender's pid on every message, and only the router's is taken.
//!
//! The router is whoever the registry names. Only the image's own router
//! can hold that name: the publisher gate refuses a capsule that declares
//! any name of the image's own systems.nonos capsules (L52), so a holder is
//! the router, restarted or not. The pid named is kept, and the registry is
//! asked again only when a frame comes from another pid.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Router {
    /// Not looked up yet, or no router running when last asked.
    Unknown,
    /// The router's pid. Pids are never handed out twice in a boot, so a
    /// frame stamped with it is that process's.
    Pinned(u32),
}

impl Router {
    /// After asking the registry, which names `registered` as the holder.
    pub fn after_lookup(self, registered: Option<u32>) -> Router {
        match registered {
            Some(p) if p != 0 => Router::Pinned(p),
            _ => Router::Unknown,
        }
    }

    pub fn believes(self, sender: u32) -> bool {
        matches!(self, Router::Pinned(p) if p == sender)
    }

    /// The token for a frame from `sender`, when the router is believed.
    pub fn frame(self, sender: u32) -> Option<Routed> {
        self.believes(sender).then_some(Routed(()))
    }

    /// Kept in one word: 0 not looked up, else the pid.
    pub fn word(self) -> u32 {
        match self {
            Router::Unknown => 0,
            Router::Pinned(p) => p,
        }
    }

    pub fn from_word(w: u32) -> Router {
        match w {
            0 => Router::Unknown,
            p => Router::Pinned(p),
        }
    }
}

/// What a handler is given for a frame the router sent. Only `Router::frame`
/// makes one, so code that needs it can only be reached from router input:
/// a message from any other pid never carries it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Routed(());
