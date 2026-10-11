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

//! Starting and stopping the driver's stream.

use alloc::vec;

use super::call::call;
use super::client::Sink;
use super::wire;

impl Sink {
    pub fn stream_start(&self, request_id: u32) -> bool {
        let mut tx = [0u8; wire::HDR_LEN];
        self.control(wire::start_request(request_id, &mut tx), &tx)
    }

    pub fn stream_stop(&self, request_id: u32) -> bool {
        self.stop_at(self.port.get(), request_id)
    }

    pub(super) fn stop_at(&self, port: u32, request_id: u32) -> bool {
        let mut tx = [0u8; wire::HDR_LEN];
        control(port, wire::stop_request(request_id, &mut tx), &tx)
    }

    fn control(&self, n: usize, tx: &[u8]) -> bool {
        control(self.port.get(), n, tx)
    }
}

fn control(port: u32, n: usize, tx: &[u8]) -> bool {
    if n == 0 {
        return false;
    }
    let mut rx = vec![0u8; wire::HDR_LEN + wire::STATUS_LEN];
    match call(port, &tx[..n], &mut rx) {
        Some(got) => wire::reply_ok(&rx[..got]),
        None => false,
    }
}
