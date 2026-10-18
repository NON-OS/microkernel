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

//! One look at every registered host's root ports, fastest host first:
//! which may be asked next, and whether any is still open. While the port a
//! device was let go on still lists it, that port alone is asked, on
//! whichever host (`release`); a port no longer listed is forgotten.

use alloc::vec::Vec;

use super::ports::Ports;
use super::scanner::Scanner;
use crate::xhci::{connected_ports, Port};

#[derive(Default)]
pub(super) struct Look {
    /// Each host that listed its ports, with them, and the errno one failed
    /// with.
    pub lists: Vec<(u32, Vec<Port>)>,
    pub failed: Option<i32>,
    pub connected: usize,
    pub open: bool,
    /// The host and port to ask now; the let-go device's, while it stays.
    pub next: Option<(u32, u8)>,
    pub left: Option<(u32, u8)>,
}

impl Scanner {
    pub(super) fn look(&mut self) -> Look {
        let mut look = Look::default();
        self.hosts.each(|xhci, _| match connected_ports(xhci) {
            Ok(listed) => look.lists.push((xhci, listed)),
            Err(e) => look.failed = Some(e),
        });
        let lists = core::mem::take(&mut look.lists);
        let on =
            |&(x, p): &(u32, u8)| listed(&lists, x).is_some_and(|l| l.iter().any(|q| q.id == p));
        look.left = self.left_on.filter(on);
        self.hosts.each(|xhci, ports| {
            let Some(listed) = listed(&lists, xhci) else { return };
            ports.forget_gone(listed);
            look.connected += listed.len();
            let only =
                look.left.filter(|l| l.0 == xhci).and_then(|l| listed.iter().find(|p| p.id == l.1));
            if look.left.is_some() && only.is_none() {
                return;
            }
            let (next, open) = ports.next(only.map_or(listed, core::slice::from_ref));
            look.open |= open;
            look.next = look.next.or(next.map(|port| (xhci, port)));
        });
        look.lists = lists;
        look
    }

    /// The ports given up on for now, of every host in `lists`.
    pub(super) fn closed(&mut self, lists: &[(u32, Vec<Port>)]) -> usize {
        let mut closed = 0;
        self.hosts.each(|xhci, ports| {
            closed += listed(lists, xhci).map_or(0, |l| usize::from(ports.closed(l)));
        });
        closed
    }

    /// The ports of host `xhci`, changed by `f`.
    pub(super) fn ports_of(&mut self, xhci: u32, f: impl FnOnce(&mut Ports)) {
        let mut f = Some(f);
        self.hosts.first(|port, ports| (port == xhci).then(|| f.take().map(|f| f(ports))));
    }
}

fn listed(lists: &[(u32, Vec<Port>)], xhci: u32) -> Option<&[Port]> {
    lists.iter().find(|(h, _)| *h == xhci).map(|(_, l)| &l[..])
}
