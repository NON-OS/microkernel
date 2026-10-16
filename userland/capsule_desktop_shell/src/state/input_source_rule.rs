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
//! The router is the first holder of its name the shell finds. That one is
//! the image's own: the kernel starts it before the shell, and nothing is
//! loaded from the store until the shell runs. A later holder is not
//! believed. The registry binds a name to whoever's signed manifest declares
//! it, and a publisher's capsule may declare this one once the router has
//! exited, so a new holder proves nothing. Once the first router is gone,
//! input is refused until the shell starts again.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Router {
    /// Not looked up yet.
    Unknown,
    /// The router's pid. Pids are never handed out twice in a boot, so a
    /// frame stamped with it is that process's.
    Pinned(u32),
    /// The pinned router left the registry: nothing is believed.
    Lost,
}

impl Router {
    /// After asking the registry, which names `registered` as the holder.
    pub fn after_lookup(self, registered: Option<u32>) -> Router {
        match (self, registered) {
            (Router::Unknown, Some(p)) if p != 0 && p != u32::MAX => Router::Pinned(p),
            (Router::Unknown, _) => Router::Unknown,
            (Router::Pinned(p), Some(q)) if p == q => self,
            (Router::Pinned(_), _) | (Router::Lost, _) => Router::Lost,
        }
    }

    pub fn believes(self, sender: u32) -> bool {
        matches!(self, Router::Pinned(p) if p == sender)
    }

    /// Kept in one word: 0 not looked up, u32::MAX lost (a pid the kernel
    /// never hands out: its counter wraps before it), else the pid.
    pub fn word(self) -> u32 {
        match self {
            Router::Unknown => 0,
            Router::Pinned(p) => p,
            Router::Lost => u32::MAX,
        }
    }

    pub fn from_word(w: u32) -> Router {
        match w {
            0 => Router::Unknown,
            u32::MAX => Router::Lost,
            p => Router::Pinned(p),
        }
    }
}
