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

//! The parser's states and the transitions every state shares.

use alloc::vec::Vec;

use super::handler::Handler;
use super::seq::Seq;
use crate::utf8::Utf8;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub(super) enum State {
    #[default]
    Ground,
    Escape,
    EscapeInter,
    CsiEntry,
    CsiParam,
    CsiInter,
    CsiIgnore,
    DcsEntry,
    DcsParam,
    DcsInter,
    DcsPass,
    DcsIgnore,
    Osc,
    /// SOS, PM and APC: read to the terminator and dropped.
    Ignored,
}

#[derive(Default)]
pub struct Parser {
    pub(super) state: State,
    pub(super) seq: Seq,
    pub(super) utf8: Utf8,
    /// OSC or DCS payload being collected.
    pub(super) buf: Vec<u8>,
    /// The payload outgrew its ceiling; it is dropped when it ends.
    pub(super) buf_full: bool,
}

impl Parser {
    pub fn new() -> Parser {
        Parser::default()
    }

    pub fn feed<H: Handler>(&mut self, h: &mut H, bytes: &[u8]) {
        for &b in bytes {
            self.advance(h, b);
        }
    }
}
