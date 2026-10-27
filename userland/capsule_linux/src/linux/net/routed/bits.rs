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

//! What poll says of a stream this capsule carries, as Linux's tcp_poll
//! says it: nothing while it opens, readable once bytes or the end have
//! come, writable while the queue has room, and the endpoint's replies
//! readable before the stream's.

use super::super::sock::End;
use super::drive::{drive, handle};
use super::entry::Phase;
use super::link::{Link, TX_MAX};
use super::table;

const POLLIN: u16 = 0x001;
const POLLOUT: u16 = 0x004;
const POLLERR: u16 = 0x008;
const POLLHUP: u16 = 0x010;
const POLLRDHUP: u16 = 0x2000;

/// poll's bits, as Linux's tcp_poll gives them.
pub fn bits(id: u32) -> Option<u16> {
    let h = handle(id)?;
    drive(id, false);
    table::with(h, |e| {
        let said = if e.out.is_empty() { 0 } else { POLLIN };
        said | match &e.phase {
            Phase::Asking(_) => POLLOUT,
            Phase::Route { .. } | Phase::Anyone { .. } | Phase::Nym { .. } => 0,
            Phase::Open(link) => link.bits(),
            Phase::Closed => POLLIN | POLLOUT | POLLRDHUP | POLLHUP,
            Phase::Failed(_) => POLLOUT | POLLERR | POLLHUP,
        }
    })
}

impl Link {
    pub fn bits(&self) -> u16 {
        let done = self.end.is_some();
        let mut set = if self.tx.len() < TX_MAX || self.broken { POLLOUT } else { 0 };
        if !self.rx.is_empty() || done {
            set |= POLLIN;
        }
        if done {
            set |= POLLRDHUP;
        }
        if matches!(self.end, Some(End::Error(_))) {
            set |= POLLERR | POLLHUP;
        }
        set
    }
}
