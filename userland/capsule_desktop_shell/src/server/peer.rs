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
//! The two peers whose messages steer the shell without a request: the
//! input router (keys, pointer, the power and volume keys, the process
//! manager chord) and the window manager (windows opened, closed, made full
//! screen). Before this check any capsule could post either kind.

use core::sync::atomic::{AtomicU32, Ordering};

use nonos_libc::mk_service_lookup;

use super::peer_rule::from_peer;

pub struct Peer {
    service: &'static [u8],
    pid: AtomicU32,
}

pub static INPUT_ROUTER: Peer = Peer::new(b"input_router");
pub static WM: Peer = Peer::new(b"wm");

impl Peer {
    const fn new(service: &'static [u8]) -> Self {
        Self { service, pid: AtomicU32::new(0) }
    }

    /// True when `sender` is this peer's process.
    pub fn sent(&self, sender: u32) -> bool {
        let known = self.pid.load(Ordering::Relaxed);
        let (ok, pid) = from_peer(sender, known, || registered_pid(self.service));
        self.pid.store(pid, Ordering::Relaxed);
        ok
    }
}

fn registered_pid(name: &[u8]) -> u32 {
    let (mut port, mut pid) = (0u32, 0u32);
    let rc = mk_service_lookup(name.as_ptr(), name.len(), &mut port, &mut pid);
    if rc == 0 {
        pid
    } else {
        0
    }
}
