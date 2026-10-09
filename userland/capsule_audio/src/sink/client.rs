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

use alloc::vec;

use nonos_libc::{mk_idle_ms, mk_ipc_call_timeout, mk_service_lookup};

use super::wire;

const DRIVER_NAME: &[u8] = b"driver.hda0";
/// The kernel registers driver.hda0 when it spawns the HD Audio driver,
/// before this capsule, and drops it when that driver exits (no controller,
/// or one that never came up). The lookup is retried for about two seconds,
/// asleep between tries; it used to be 2000 tries a yield apart, which on an
/// idle machine is a spin and on a busy one has no fixed length.
const LOOKUP_RETRIES: u32 = 100;
const LOOKUP_PAUSE_MS: u64 = 20;
const CALL_TIMEOUT_MS: u64 = 1000;

pub struct Sink {
    port: u32,
}

impl Sink {
    pub fn resolve() -> Option<Sink> {
        for tries in 0..LOOKUP_RETRIES {
            if tries > 0 {
                let _ = mk_idle_ms(LOOKUP_PAUSE_MS);
            }
            let mut port = 0u32;
            let mut pid = 0u32;
            let r = mk_service_lookup(DRIVER_NAME.as_ptr(), DRIVER_NAME.len(), &mut port, &mut pid);
            if r == 0 && port != 0 {
                return Some(Sink { port });
            }
        }
        None
    }

    pub fn write_pcm(&self, pcm: &[u8], request_id: u32) -> bool {
        self.write_pcm_status(pcm, request_id) == 0
    }

    pub fn write_pcm_status(&self, pcm: &[u8], request_id: u32) -> i32 {
        let mut tx = vec![0u8; wire::HDR_LEN + pcm.len()];
        let n = wire::request(request_id, pcm, &mut tx);
        if n == 0 {
            return i32::MIN;
        }
        let mut rx = vec![0u8; wire::HDR_LEN + wire::STATUS_LEN];
        let got = mk_ipc_call_timeout(
            self.port as u64,
            tx.as_ptr(),
            n,
            rx.as_mut_ptr(),
            rx.len(),
            CALL_TIMEOUT_MS,
        );
        if got <= 0 {
            return i32::MIN;
        }
        wire::reply_status(&rx[..got as usize])
    }

    pub fn stream_start(&self, request_id: u32) -> bool {
        let mut tx = [0u8; wire::HDR_LEN];
        self.control(wire::start_request(request_id, &mut tx), &tx)
    }

    pub fn stream_stop(&self, request_id: u32) -> bool {
        let mut tx = [0u8; wire::HDR_LEN];
        self.control(wire::stop_request(request_id, &mut tx), &tx)
    }

    /// The driver's verdict on this machine's audio and its output flags,
    /// or none when it does not answer.
    pub fn output_status(&self, request_id: u32) -> Option<(u32, u32)> {
        let mut tx = [0u8; wire::HDR_LEN];
        let n = wire::output_status_request(request_id, &mut tx);
        let mut rx = [0u8; wire::OUTPUT_REPLY_LEN];
        let got = mk_ipc_call_timeout(
            self.port as u64,
            tx.as_ptr(),
            n,
            rx.as_mut_ptr(),
            rx.len(),
            CALL_TIMEOUT_MS,
        );
        if got <= 0 {
            return None;
        }
        wire::read_output_status(&rx[..got as usize])
    }

    fn control(&self, n: usize, tx: &[u8]) -> bool {
        if n == 0 {
            return false;
        }
        let mut rx = vec![0u8; wire::HDR_LEN + wire::STATUS_LEN];
        let got = mk_ipc_call_timeout(
            self.port as u64,
            tx.as_ptr(),
            n,
            rx.as_mut_ptr(),
            rx.len(),
            CALL_TIMEOUT_MS,
        );
        if got <= 0 {
            return false;
        }
        wire::reply_ok(&rx[..got as usize])
    }
}
