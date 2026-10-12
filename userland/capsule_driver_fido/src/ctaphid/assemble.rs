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
//! message as broken rather than splicing bytes into it.

use alloc::vec::Vec;

use super::frame::{Cid, CMD_ERROR, CMD_KEEPALIVE, MESSAGE_MAX};
use super::heard::Heard;

pub struct Assembler {
    cid: Cid,
    cmd: u8,
    need: usize,
    got: Vec<u8>,
    seq: Option<u8>,
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
            let need = u16::from_be_bytes([p[5], p[6]]) as usize;
            match p[4] {
                CMD_KEEPALIVE => return Heard::Keepalive(p[7]),
                CMD_ERROR => return Heard::Error(p[7]),
                _ if need > MESSAGE_MAX => return self.broken(),
                cmd => (self.cmd, self.need, self.seq) = (cmd, need, Some(0)),
            }
            self.got.clear();
            self.take(&p[7..])
        } else if self.seq == Some(p[4]) {
            self.seq = Some(p[4] + 1);
            self.take(&p[5..])
        } else {
            self.broken()
        }
    }

    fn take(&mut self, data: &[u8]) -> Heard {
        let n = (self.need - self.got.len()).min(data.len());
        self.got.extend_from_slice(&data[..n]);
        if self.got.len() < self.need {
            return Heard::Nothing;
        }
        self.seq = None;
        Heard::Message(self.cmd, core::mem::take(&mut self.got))
    }

    fn broken(&mut self) -> Heard {
        (self.seq, self.need) = (None, 0);
        self.got.clear();
        Heard::Broken
    }
}
