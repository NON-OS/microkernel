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
//! One payload into the frame being put together (frame.rs).

use crate::frame::{Assembler, Payload};
use crate::header::{header, EOF, ERR, FID};

impl Assembler {
    /// One payload as the endpoint delivered it; an empty one is none.
    pub fn push(&mut self, p: &[u8]) -> Payload {
        if p.is_empty() {
            return Payload::More;
        }
        let Some((hlen, info)) = header(p) else {
            self.bad = true;
            return Payload::More;
        };
        let fid = info & FID != 0;
        let mut out = Payload::More;
        if self.fid.is_some_and(|f| f != fid) {
            // A new frame began before the last one ended: that one was
            // cut short and is dropped.
            self.buf.clear();
            self.bad = false;
        }
        self.fid = Some(fid);
        self.bad |= info & ERR != 0 || self.buf.len() + p.len() - hlen > self.max;
        if !self.bad {
            self.buf.extend_from_slice(&p[hlen..]);
        }
        if info & EOF != 0 {
            out = self.finish();
        }
        out
    }
}
