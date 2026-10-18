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

//! Every USB host a class driver searches, fastest first, each with what
//! the driver keeps for it (its ports' tries, its devices). A host is
//! looked up until it has registered, so one that comes up late is searched
//! from the next pass on; a device found through a host is served through
//! it. Pure over the lookup, so the proofs run it with modelled hosts.
//! usb_hid and usb_msc mount this file by path, and usb_msc hosts_first.rs
//! beside it, so each driver carries only what it calls.

use alloc::vec::Vec;

// The fields are hosts_first.rs's too.
pub(super) struct Host<T> {
    pub(super) name: &'static str,
    pub(super) port: Option<u32>,
    pub(super) keep: T,
}

pub struct Hosts<T> {
    pub(super) list: Vec<Host<T>>,
}

impl<T> Hosts<T> {
    /// `names` in the order they are searched, each with a fresh `keep()`.
    pub fn new(names: &[&'static str], mut keep: impl FnMut() -> T) -> Self {
        let list = names.iter().map(|&name| Host { name, port: None, keep: keep() }).collect();
        Hosts { list }
    }

    /// Look up each host not yet registered; returns how many are known.
    pub fn find(&mut self, mut lookup: impl FnMut(&str) -> Option<u32>) -> usize {
        for h in self.list.iter_mut().filter(|h| h.port.is_none()) {
            h.port = lookup(h.name);
        }
        self.list.iter().filter(|h| h.port.is_some()).count()
    }

    /// Visit every known host in order.
    pub fn each(&mut self, mut visit: impl FnMut(u32, &mut T)) {
        for h in self.list.iter_mut() {
            if let Some(port) = h.port {
                visit(port, &mut h.keep);
            }
        }
    }
}
