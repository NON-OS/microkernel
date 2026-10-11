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

use super::call::call;
use super::lookup::resolve;
use super::wire;

const DRIVER_NAME: &[u8] = b"driver.hda0";

pub struct Sink {
    pub(super) port: u32,
}

impl Sink {
    pub fn resolve() -> Option<Sink> {
        resolve(DRIVER_NAME).map(|port| Sink { port })
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
        match call(self.port, &tx[..n], &mut rx) {
            Some(got) => wire::reply_status(&rx[..got]),
            None => i32::MIN,
        }
    }

    /// The driver's verdict on this machine's audio and its output flags,
    /// or none when it does not answer.
    pub fn output_status(&self, request_id: u32) -> Option<(u32, u32)> {
        let mut tx = [0u8; wire::HDR_LEN];
        let n = wire::output_status_request(request_id, &mut tx);
        let mut rx = [0u8; wire::OUTPUT_REPLY_LEN];
        let got = call(self.port, &tx[..n], &mut rx)?;
        wire::read_output_status(&rx[..got])
    }
}
