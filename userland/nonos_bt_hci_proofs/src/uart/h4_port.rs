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

//! The simulated controller behind an H4 port: each command it is sent,
//! after its packet indicator, draws the sim's answer, and each event goes
//! back after the event indicator (Vol 4, Part A, 2). The port hands the
//! host at most `chunk` octets per read, so packets arrive cut.

use std::collections::VecDeque;

use crate::sim::config::Config;
use crate::sim::controller::respond;
use nonos_bt_hci::uart::UartPort;

pub struct H4Port {
    pub cfg: Config,
    pub to_host: VecDeque<u8>,
    pub written: Vec<Vec<u8>>,
    pub chunk: usize,
    pub clock: u64,
}

impl H4Port {
    pub fn new(cfg: Config, chunk: usize) -> Self {
        Self { cfg, to_host: VecDeque::new(), written: Vec::new(), chunk, clock: 0 }
    }

    /// Put these octets on the line toward the host.
    pub fn line(&mut self, octets: &[u8]) {
        self.to_host.extend(octets);
    }
}

impl UartPort for H4Port {
    fn write(&mut self, data: &[u8]) -> Result<(), i32> {
        self.written.push(data.to_vec());
        if data.first() == Some(&0x01) {
            for ev in respond(&self.cfg, &data[1..]) {
                self.to_host.push_back(0x04);
                self.to_host.extend(ev);
            }
        }
        Ok(())
    }
    fn read(&mut self, out: &mut [u8]) -> Result<usize, i32> {
        let n = out.len().min(self.chunk).min(self.to_host.len());
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
