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

//! READ: what the guest wrote, at most `want`. With nothing there yet it is
//! held, one at a time, until the guest writes or `wait_ms` passes, then
//! answered empty; CLOSED once the guest closed and all it wrote was read.

use alloc::vec::Vec;

use nonos_serve_wire::{EBUSY, OK};

use super::conn::{take, Conn, Held};
use crate::linux::net::sock::Socks;

impl Conn {
    /// READ (`seq`, `want`, `wait_ms`) as `request` asks it: its answer, or
    /// None while it is held.
    pub fn read(
        &mut self,
        t: &mut Socks,
        request: u64,
        ask: (u32, u32, u32),
        now: u64,
    ) -> Option<(u8, Vec<u8>)> {
        let (seq, want, wait) = ask;
        if let Some((_, status, body)) = self.read.as_ref().filter(|r| r.0 == seq) {
            return Some((*status, body.clone()));
        }
        match self.held.as_mut() {
            /* Its asker's call ran out and it asks again: the new call waits on. */
            Some(h) if h.seq == seq => h.request = request,
            Some(_) => return Some((EBUSY, Vec::new())),
            None => match take(t, self.end, want) {
                Some(said) => return Some(self.keep(seq, said)),
                None if wait == 0 => return Some(self.keep(seq, (OK, Vec::new()))),
                None => {
                    let until = now.saturating_add(u64::from(wait));
                    self.held = Some(Held { request, seq, want, until });
                }
            },
        }
        None
    }

    /// The held READ's request and answer, once the guest wrote or closed or
    /// its wait ran out.
    pub fn settle(&mut self, t: &mut Socks, now: u64) -> Option<(u64, u8, Vec<u8>)> {
        let h = self.held?;
        let said =
            take(t, self.end, h.want).or_else(|| (now >= h.until).then(|| (OK, Vec::new())))?;
        self.held = None;
        let (status, body) = self.keep(h.seq, said);
        Some((h.request, status, body))
    }
}
