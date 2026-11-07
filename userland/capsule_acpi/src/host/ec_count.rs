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

//! One EC transaction as the trace sees it: the port accesses it made, so a
//! timeout says where it stopped (before the command, after it, or after
//! the address, waiting for an answer) and how many bytes left in the
//! output buffer were read away first (bytes nobody of ours asked for: the
//! sign of firmware's SMI handler on the same ports).

use crate::events_io::PortIo;
use nonos_aml::ec::EcPorts;
use nonos_aml::events::EventIo;

pub struct Counted {
    pub ports: EcPorts,
    /// Writes to either port: the command, the address, the data.
    pub writes: u32,
    /// Data-port reads before the first write: stale bytes flushed.
    pub stale: u32,
    pub status_reads: u32,
}

impl Counted {
    pub fn new(ports: EcPorts) -> Self {
        Counted { ports, writes: 0, stale: 0, status_reads: 0 }
    }

    /// Where a timed-out transaction stopped.
    pub fn stopped(&self, write: bool) -> &'static [u8] {
        match (self.writes, write) {
            (0, _) => b"before-command",
            (1, _) => b"after-command",
            (2, false) => b"after-address",
            (2, true) => b"before-data",
            _ => b"after-data",
        }
    }
}

impl EventIo for Counted {
    fn read(&mut self, port: u16, width: u8) -> Option<u16> {
        if port == self.ports.command {
            self.status_reads += 1;
        } else if self.writes == 0 {
            self.stale += 1;
        }
        PortIo.read(port, width)
    }
    fn write(&mut self, port: u16, width: u8, value: u16) -> bool {
        self.writes += 1;
        PortIo.write(port, width, value)
    }
    fn sleep(&mut self, ms: u64) {
        PortIo.sleep(ms)
    }
    fn now_ms(&mut self) -> u64 {
        PortIo.now_ms()
    }
}
