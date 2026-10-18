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
//! The one request in flight on our channel. It runs until the key
//! answers, fails it, or goes quiet too long. One that ends any other way
//! (its client gone, the key quiet) costs the channel: the driver cancels
//! it and takes a new one, so its late answer, on the old channel, is never
//! taken for the answer to the next request.

use alloc::vec::Vec;

use super::assemble::Assembler;
use super::frame::{Cid, CMD_CBOR};
use super::heard::Heard;
use super::talk::E_IO;

pub const E_AGAIN: i32 = -11;
/// An errno, and the key's CTAPHID error code or keep-alive status.
pub type Fault = (i32, Option<u8>);

#[derive(Default)]
pub enum Job {
    #[default]
    Idle,
    Running(Assembler, u8),
    Done(Result<Vec<u8>, Fault>),
}

impl Job {
    pub fn start(cid: Cid) -> Self {
        Job::Running(Assembler::new(cid), 0)
    }

    pub fn busy(&self) -> bool {
        matches!(self, Job::Running(..))
    }

    /// One report from the key; whether it was word from it.
    pub fn hear(&mut self, p: &[u8]) -> bool {
        let heard = match self {
            Job::Running(asm, _) => asm.feed(p),
            _ => return false,
        };
        let end = match heard {
            Heard::Keepalive(s) => {
                if let Job::Running(_, status) = self {
                    *status = s;
                }
                return true;
            }
            Heard::Message(CMD_CBOR, data) => Ok(data),
            Heard::Error(code) => Err((E_IO, Some(code))),
            Heard::Broken => Err((E_IO, None)),
            _ => return false,
        };
        *self = Job::Done(end);
        true
    }
}
