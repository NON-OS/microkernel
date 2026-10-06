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

//! OSC and DCS payloads: collected up to a ceiling and handed over whole
//! when they end, or dropped whole when they outgrew it.

use super::handler::Handler;
use super::machine::{Parser, State};
use crate::limits::{MAX_DCS, MAX_OSC};

impl Parser {
    fn keep(&mut self, b: u8, max: usize) {
        if self.buf.len() < max {
            self.buf.push(b);
        } else {
            self.buf_full = true;
        }
    }

    pub(super) fn osc_byte<H: Handler>(&mut self, h: &mut H, b: u8) {
        match b {
            0x07 => {
                self.end_osc(h);
                self.state = State::Ground;
            }
            0x00..=0x1F => {}
            _ => self.keep(b, MAX_OSC),
        }
    }

    pub(super) fn end_osc<H: Handler>(&mut self, h: &mut H) {
        if !self.buf_full {
            h.osc(&self.buf);
        }
        self.buf.clear();
    }

    pub(super) fn dcs_pass(&mut self, b: u8) {
        if b != 0x7F {
            self.keep(b, MAX_DCS);
        }
    }

    pub(super) fn end_dcs<H: Handler>(&mut self, h: &mut H) {
        if !self.buf_full && !self.seq.truncated() {
            h.dcs(&self.seq, &self.buf);
        }
        self.buf.clear();
    }
}
