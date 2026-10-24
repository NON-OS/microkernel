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
//! The reports a key sends, put back together. Another channel's pass
//! by; a continuation out of sequence, or with no message begun, ends the
//! message as broken rather than splicing bytes into it, as does a
//! message opened again before the last one ended (opening.rs judges).

use alloc::vec::Vec;

use super::frame::Cid;
use super::heard::Heard;
use super::opening::{opening, Opening};

pub struct Assembler {
    pub(super) cid: Cid,
    pub(super) cmd: u8,
    pub(super) need: usize,
    pub(super) got: Vec<u8>,
    pub(super) seq: Option<u8>,
}

impl Assembler {
    pub fn new(cid: Cid) -> Self {
        Self { cid, cmd: 0, need: 0, got: Vec::new(), seq: None }
    }

    pub fn feed(&mut self, p: &[u8]) -> Heard {
        if p.len() < 7 || p[..4] != self.cid {
            return Heard::Nothing;
        }
        if p[4] & 0x80 != 0 {
            match opening(p) {
                Opening::Keepalive(s) => Heard::Keepalive(s),
                Opening::Error(code) => Heard::Error(code),
                Opening::Ignore => Heard::Nothing,
                // Opened again before the last one ended: neither is taken.
                Opening::Begin(..) if self.seq.is_some() => self.broken(),
                Opening::Broken => self.broken(),
                // With no message under way `got` is empty: take() empties
                // it on finishing and broken() clears it.
                Opening::Begin(cmd, need) => {
                    (self.cmd, self.need, self.seq) = (cmd, need, Some(0));
                    self.take(&p[7..])
                }
            }
        } else if self.seq == Some(p[4]) {
            self.seq = Some(p[4] + 1);
            self.take(&p[5..])
        } else {
            self.broken()
        }
    }
}
