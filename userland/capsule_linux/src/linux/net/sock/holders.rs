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

//! Which processes hold a socket. A descriptor copied by fork is held by the
//! child as well; the socket is let go when the last holder closes it or
//! ends. A process that ends without closing lets go of everything it held
//! when the family drops it, which is what `Holder` does.

use alloc::vec::Vec;

use super::cell::with;
use crate::linux::guest::{Fd, Kind};

/// Carried by each hosted process, as the pid its holdings are kept under.
pub struct Holder {
    pid: u32,
}

impl Holder {
    pub fn new(pid: u32) -> Holder {
        Holder { pid }
    }

    /// A forked child holds every socket its parent's descriptors name.
    pub fn fork(&self, child: u32, fds: &[Fd]) {
        with(|t| {
            for f in fds.iter().filter(|f| f.kind == Kind::Socket) {
                if let Some(s) = t.get_mut(f.handle) {
                    if !s.holders.contains(&child) {
                        s.holders.push(child);
                    }
                }
            }
        });
    }
}

impl Drop for Holder {
    fn drop(&mut self) {
        let pid = self.pid;
        with(|t| {
            let held: Vec<u32> =
                t.iter().filter(|(_, s)| s.holders.contains(&pid)).map(|(i, _)| i).collect();
            for id in held {
                t.release(id, pid);
            }
        });
    }
}
