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

//! A boxed run's report, taken only from frames its capsule wrote. The
//! terminal names the run a key (`--box-key=<hex>`, before its words); the
//! capsule opens every report frame with TAG and that key. The guest writes
//! the same channel but is never told the key, so nothing it prints is
//! taken for the report, and the report is drawn only once it has ended.

use alloc::vec::Vec;

/// nonos_libc::TAG_BOX; the terminal holds the two equal at build time.
pub const TAG: u8 = 3;
pub const KEY_LEN: usize = 16;
/// The most of a report kept: a report is a few hundred bytes.
pub const HELD_MAX: usize = 64 * 1024;
pub struct BoxHold {
    key: [u8; KEY_LEN],
    held: Vec<u8>,
    /// The run's exit status, once it ended with the screen another's.
    pub ended: Option<i32>,
}

impl BoxHold {
    pub fn new(key: [u8; KEY_LEN]) -> Self {
        Self { key, held: Vec::new(), ended: None }
    }

    /// Whether `msg` is a frame of the report, kept if it is.
    pub fn take(&mut self, msg: &[u8]) -> bool {
        let Some((&TAG, rest)) = msg.split_first() else { return false };
        let Some(body) = rest.strip_prefix(&self.key[..]) else { return false };
        let room = HELD_MAX.saturating_sub(self.held.len());
        self.held.extend_from_slice(&body[..body.len().min(room)]);
        true
    }

    /// Whether the report came whole: its `[BOX]` line is its last.
    pub fn complete(&self) -> bool {
        self.lines().1.is_some()
    }

    /// The report's lines for the screen, and its `[BOX]` line for the log.
    pub fn lines(&self) -> (Vec<&[u8]>, Option<&[u8]>) {
        let mut shown = Vec::new();
        let mut line = None;
        for l in self.held.split(|b| *b == b'\n').filter(|l| !l.is_empty()) {
            match l.starts_with(b"[BOX] ") {
                true => line = Some(l),
                false => shown.push(l),
            }
        }
        (shown, line)
    }
}
