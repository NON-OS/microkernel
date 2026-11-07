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

//! Frames taken from the personality's inbox by another reader and kept for
//! the input pump. While SERVE_ONE serves a listener, its pump is the one
//! reader of the inbox, and a frame that owes no answer (the input router's,
//! the window manager's, the desktop shell's) is set aside here rather than
//! dropped. Pure, so a host proof holds it (capsule_linux_proofs,
//! inbox_aside_tests).

use alloc::collections::VecDeque;
use alloc::vec::Vec;

/// Frames kept at most. A deeper backlog loses the newest, as a full inbox
/// would, so what is kept stays in the order it came.
pub const KEPT_MAX: usize = 128;

pub struct Aside {
    frames: VecDeque<(u32, Vec<u8>)>,
}

impl Aside {
    pub const fn new() -> Aside {
        Aside { frames: VecDeque::new() }
    }

    /// Keep `frame` from `sender`, unless the input pump would not believe
    /// that sender, the frame is longer than `cap`, the longest frame the
    /// pump reads, or KEPT_MAX are kept already. A frame from anyone else
    /// takes no room, so no other sender can crowd out the router's.
    pub fn put(
        &mut self,
        sender: u32,
        frame: &[u8],
        cap: usize,
        believed: impl Fn(u32) -> bool,
    ) -> bool {
        if frame.is_empty() || frame.len() > cap || self.frames.len() >= KEPT_MAX {
            return false;
        }
        if !believed(sender) {
            return false;
        }
        self.frames.push_back((sender, frame.to_vec()));
        true
    }

    /// The oldest frame kept, copied into `into`: its length and sender. A
    /// frame longer than `into` is not taken.
    pub fn take(&mut self, into: &mut [u8]) -> Option<(usize, u32)> {
        let (sender, frame) = self.frames.front()?;
        let n = frame.len();
        into.get_mut(..n)?.copy_from_slice(frame);
        let sender = *sender;
        self.frames.pop_front();
        Some((n, sender))
    }
}
