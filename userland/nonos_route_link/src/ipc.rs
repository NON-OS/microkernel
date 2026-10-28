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

/*
 * A proxy's service port, reached by a bounded IPC call. The kernel stamps
 * each call with its own token, so an answer that arrives after its caller
 * stopped waiting is discarded rather than handed to the next call. A
 * carrier for a named stream names it in every frame it carries (`frame_on`).
 */

use nonos_libc::{mk_idle_ms, mk_ipc_call_timeout, mk_uptime_ms};

use crate::carrier::Carrier;

pub struct Ipc {
    port: u32,
    /* The stream every frame names, 0 for the one a frame naming none is. */
    stream: u32,
}

impl Ipc {
    pub fn new(port: u32) -> Ipc {
        Ipc { port, stream: 0 }
    }

    pub fn on(port: u32, stream: u32) -> Ipc {
        Ipc { port, stream }
    }
}

impl Carrier for Ipc {
    fn call(&mut self, frame: &[u8], wait_ms: u64, into: &mut [u8]) -> i64 {
        let named = crate::frame_on::on(self.stream, frame);
        let frame = named.as_deref().unwrap_or(frame);
        mk_ipc_call_timeout(
            self.port as u64,
            frame.as_ptr(),
            frame.len(),
            into.as_mut_ptr(),
            into.len(),
            wait_ms,
        )
    }

    fn now_ms(&self) -> i64 {
        mk_uptime_ms()
    }

    fn pause(&mut self, ms: u64) {
        let _ = mk_idle_ms(ms);
    }
}
