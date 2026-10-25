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


//! What a program was handed of its input before it was heard from, kept
//! so that a run started again (net_again.rs) is handed it again. A run
//! asks to start again before its program runs, so nothing kept was read.

use alloc::vec::Vec;

/// The most kept; past it, or once the program writes, nothing is.
pub const KEPT_MAX: usize = 64 * 1024;

pub struct Kept {
    msgs: Vec<Vec<u8>>,
    bytes: usize,
    off: bool,
}

impl Kept {
    pub const fn new() -> Self {
        Self { msgs: Vec::new(), bytes: 0, off: false }
    }

    /// `msg` was handed over.
    pub fn handed(&mut self, msg: &[u8]) {
        if self.off {
            return;
        }
        if self.bytes + msg.len() > KEPT_MAX {
            return self.heard();
        }
        self.bytes += msg.len();
        self.msgs.push(msg.to_vec());
    }

    /// The program wrote: it runs, and what it was handed may be read.
    pub fn heard(&mut self) {
        self.off = true;
        self.msgs = Vec::new();
        self.bytes = 0;
    }

    /// Everything kept, oldest first; nothing is kept after.
    pub fn take(&mut self) -> Vec<Vec<u8>> {
        let msgs = core::mem::take(&mut self.msgs);
        self.heard();
        msgs
    }
}
