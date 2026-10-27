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

//! An open stream through net.socks5, carried in slices so no guest waits
//! on another's: each call to the proxy waits at most SLICE_MS, and one
//! left unanswered is asked again later under the same number. A frame
//! unanswered goes again with the very same bytes, so the proxy hands back
//! the answer it kept rather than carrying them twice; nothing is read
//! while one is out, since an empty frame under its number would take that
//! answer for a poll's and the bytes would go again.

use alloc::vec::Vec;

use nonos_route_link::CARRY_MAX;

use super::super::sock::End;
use super::link::Link;
use super::nym::SLICE_MS;
use crate::linux::abi::errno::{ECONNRESET, EIO};

/// The most one read from the proxy takes into the guest's buffer.
const RX_MAX: usize = 32 << 10;

impl Link {
    /// Carry what is queued and take in what has come, a call or two.
    pub fn pump(&mut self) {
        /* Nothing reaches a network from a family that holds a model. */
        if crate::linux::file::models::held() && self.end.is_none() {
            (self.end, self.broken) = (Some(End::Error(EIO)), true);
        }
        if !self.tx.is_empty() && !self.broken {
            let n = if self.offered == 0 { self.tx.len().min(CARRY_MAX) } else { self.offered };
            self.offered = n;
            match self.route.write_slice(&self.tx[..n], SLICE_MS) {
                Ok(0) => return,
                Ok(sent) => {
                    self.tx.drain(..sent.min(n));
                }
                Err(_) => (self.tx, self.broken) = (Vec::new(), true),
            }
            self.offered = 0;
        }
        if !self.rx.is_empty() || self.end.is_some() {
            return;
        }
        self.rx.resize(RX_MAX, 0);
        match self.route.read_slice(&mut self.rx, SLICE_MS) {
            Ok(n) => self.rx.truncate(n),
            Err(_) => (self.rx, self.end) = (Vec::new(), Some(End::Error(ECONNRESET))),
        }
        if self.rx.is_empty() && self.end.is_none() && self.route.ended() {
            self.end = Some(End::Eof);
        }
    }
}
