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

//! The simulated Three-wire controller's port, as the host's transport
//! sees it: what the host writes reaches the controller's side at once,
//! and each read first lets that side send what is due.

use super::h5_port::H5Port;
use nonos_bt_hci::uart::UartPort;

impl UartPort for H5Port {
    fn write(&mut self, data: &[u8]) -> Result<(), i32> {
        self.peer.feed(data, self.clock);
        self.serve();
        Ok(())
    }
    fn read(&mut self, out: &mut [u8]) -> Result<usize, i32> {
        self.serve();
        let n = out.len().min(self.to_host.len());
        for (o, b) in out.iter_mut().zip(self.to_host.drain(..n)) {
            *o = b;
        }
        Ok(n)
    }
    fn now_ms(&mut self) -> u64 {
        self.clock
    }
    fn pause(&mut self) {
        self.clock += 1;
    }
}
