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

//! A modelled embedded controller on ports 0x62 and 0x66: 256 bytes of
//! address space, a queue of query events, and an input buffer that stays
//! busy for a few status reads after each byte (or for ever, when stuck).

use crate::ec::{EcPorts, QR_EC, RD_EC, WR_EC};
use std::collections::VecDeque;

pub const PORTS: EcPorts = EcPorts { data: 0x62, command: 0x66 };

#[derive(Clone, Copy, PartialEq)]
enum Phase {
    Idle,
    ReadAddr,
    WriteAddr,
    WriteData(u8),
}

pub struct EcModel {
    pub mem: [u8; 256],
    pub events: VecDeque<u8>,
    pub delay: u32,
    pub stuck: bool,
    pub slept_ms: u64,
    pub(crate) busy: u32,
    pub(crate) out: Option<u8>,
    phase: Phase,
}

impl EcModel {
    pub fn new(delay: u32) -> Self {
        EcModel {
            mem: [0; 256],
            events: VecDeque::new(),
            delay,
            stuck: false,
            slept_ms: 0,
            busy: 0,
            out: None,
            phase: Phase::Idle,
        }
    }

    pub(crate) fn take(&mut self, port: u16, v: u8) {
        self.busy = self.delay;
        match (port, self.phase) {
            (0x66, _) if v as u16 == RD_EC => self.phase = Phase::ReadAddr,
            (0x66, _) if v as u16 == WR_EC => self.phase = Phase::WriteAddr,
            (0x66, _) if v as u16 == QR_EC => self.out = Some(self.events.pop_front().unwrap_or(0)),
            (0x62, Phase::ReadAddr) => {
                (self.out, self.phase) = (Some(self.mem[v as usize]), Phase::Idle)
            }
            (0x62, Phase::WriteAddr) => self.phase = Phase::WriteData(v),
            (0x62, Phase::WriteData(a)) => (self.mem[a as usize], self.phase) = (v, Phase::Idle),
            _ => {}
        }
    }
}
