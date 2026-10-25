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

//! The channel's line to the host through the kernel: its record's posts
//! and answers, its signal, the clock. Each call is on the channel's own
//! record, so the kernel takes only this channel's messages and hands back
//! only this channel's answers.

use nonos_libc::{
    mk_hv_post, mk_hv_recv, mk_hv_signal, mk_idle_ms, mk_uptime_ms, mk_yield, MK_HV_MESSAGE_BYTES,
};
use nonos_vmbus::msg::MESSAGE_MAX;
use nonos_vmbus::{Host, PostError};

/// HVMSG_VMBUS's message type, the SynIC header's first word.
const MESSAGE_TYPE_CHANNEL: u32 = 1;
const HEADER: usize = 16;
const E_AGAIN: i64 = -11;
/// Looks a wait takes a yield apart, for the request a host answers at
/// once, before it sleeps a millisecond between looks.
const QUICK_LOOKS: u32 = 64;

pub struct Kernel {
    pub device_id: u64,
    looks: u32,
    last_ms: u64,
}

impl Kernel {
    pub fn new(device_id: u64) -> Self {
        Self { device_id, looks: 0, last_ms: 0 }
    }
}

impl Host for Kernel {
    fn post(&mut self, m: &[u8]) -> Result<(), PostError> {
        // Connection 0: the kernel posts on the bus's own.
        match mk_hv_post(self.device_id, 0, MESSAGE_TYPE_CHANNEL, m.as_ptr(), m.len()) {
            0 => Ok(()),
            E_AGAIN => Err(PostError::Busy),
            _ => Err(PostError::Refused),
        }
    }

    fn answer(&mut self, out: &mut [u8; MESSAGE_MAX]) -> Option<usize> {
        let mut raw = [0u8; MK_HV_MESSAGE_BYTES];
        let n = mk_hv_recv(self.device_id, &mut raw);
        if n < HEADER as i64 {
            return None;
        }
        let ty = u32::from_le_bytes([raw[0], raw[1], raw[2], raw[3]]);
        if ty != MESSAGE_TYPE_CHANNEL {
            return None;
        }
        let len = ((n as usize).min(MK_HV_MESSAGE_BYTES) - HEADER).min(MESSAGE_MAX);
        out[..len].copy_from_slice(&raw[HEADER..HEADER + len]);
        Some(len)
    }

    fn signal(&mut self) {
        let _ = mk_hv_signal(self.device_id, 0);
    }

    fn now_ms(&mut self) -> u64 {
        mk_uptime_ms().max(0) as u64
    }

    fn idle(&mut self) {
        // A look long after the last is a new wait: it starts quick again.
        let now = self.now_ms();
        if now.saturating_sub(self.last_ms) > 1 {
            self.looks = 0;
        }
        self.last_ms = now;
        if self.looks < QUICK_LOOKS {
            self.looks += 1;
            let _ = mk_yield();
        } else {
            let _ = mk_idle_ms(1);
        }
    }
}
