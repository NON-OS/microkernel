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

//! Packets out of a SLIP stream (Vol 4, Part D, "SLIP Layer"). Octets before the
//! first 0xC0 are noise on the line and are passed over. A frame with an
//! escape code SLIP does not define, or longer than any Three-wire packet,
//! is dropped whole and counted: the next 0xC0 starts again cleanly, which
//! is what the delimiter is for.

use alloc::vec::Vec;

use super::slip::{unescape, END, ESC};

/// A four-octet header, 4095 octets of payload and a two-octet check.
pub const FRAME_MAX: usize = 4 + 4095 + 2;

#[derive(Default)]
pub struct Deslip {
    buf: Vec<u8>,
    inside: bool,
    escaped: bool,
    broken: bool,
    pub oof: bool,
    pub dropped: u32,
}

impl Deslip {
    /// Feed one octet; the frame it ends, if it ends a whole one.
    pub fn push(&mut self, b: u8) -> Option<Vec<u8>> {
        if b == END {
            let frame = core::mem::take(&mut self.buf);
            let whole = self.inside && !self.broken && !self.escaped && !frame.is_empty();
            if self.inside && !whole && (self.broken || self.escaped) {
                self.dropped = self.dropped.saturating_add(1);
            }
            (self.inside, self.escaped, self.broken) = (true, false, false);
            return whole.then_some(frame);
        }
        if !self.inside || self.broken {
            return None;
        }
        let octet = if self.escaped {
            self.escaped = false;
            unescape(b, self.oof)
        } else if b == ESC {
            self.escaped = true;
            return None;
        } else {
            Some(b)
        };
        match octet {
            Some(o) if self.buf.len() < FRAME_MAX => self.buf.push(o),
            _ => (self.broken, self.buf) = (true, Vec::new()),
        }
        None
    }
}
