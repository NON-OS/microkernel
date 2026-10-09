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

/* A file's bytes as they arrive from the mirror, from the byte asked for. */

use alloc::vec::Vec;

use nonos_tls::stream::Stream;

use crate::net::Link;

pub struct Body {
    pub tls: Stream,
    pub link: Link,
    /* Body bytes that came with the headers. */
    pub first: Vec<u8>,
    /* Bytes to drop first: a mirror that ignored the range sent them again. */
    pub skip: u64,
}

impl Body {
    /* What has come since the last call; empty when nothing has yet. */
    pub fn next(&mut self) -> Result<Vec<u8>, ()> {
        let mut got = if self.first.is_empty() {
            self.tls.read(&mut self.link).map_err(|_| ())?
        } else {
            core::mem::take(&mut self.first)
        };
        let drop = self.skip.min(got.len() as u64) as usize;
        self.skip -= drop as u64;
        got.drain(..drop);
        Ok(got)
    }

    /* Whether the mirror has closed the connection. */
    pub fn ended(&self) -> bool {
        self.first.is_empty() && self.tls.is_done()
    }
}
