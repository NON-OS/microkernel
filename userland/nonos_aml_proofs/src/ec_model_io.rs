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

//! The modelled EC's ports: status with IBF while busy, OBF when a byte
//! waits, SCI_EVT while events are queued; the data port hands the byte
//! over once.

use crate::ec::{IBF, OBF, SCI_EVT};
use crate::ec_model::EcModel;
use crate::events::EventIo;

impl EventIo for EcModel {
    fn read(&mut self, port: u16, _: u8) -> Option<u16> {
        match port {
            0x66 => {
                let ibf = self.stuck || self.busy > 0;
                self.busy = self.busy.saturating_sub(1);
                let evt = if self.events.is_empty() && !self.tricks.endless { 0 } else { SCI_EVT };
                let held = self.tricks.obf_stuck || !self.tricks.stale.is_empty();
                Some(
                    evt | if ibf { IBF } else { 0 }
                        | if (held || self.out.is_some()) && !ibf { OBF } else { 0 },
                )
            }
            0x62 if self.tricks.obf_stuck => Some(0xFF),
            0x62 => self.tricks.stale.pop_front().or_else(|| self.out.take()).map(u16::from),
            _ => None,
        }
    }

    fn write(&mut self, port: u16, _: u8, value: u16) -> bool {
        let known = port == 0x62 || port == 0x66;
        if known {
            self.take(port, value as u8);
        }
        known
    }

    fn sleep(&mut self, ms: u64) {
        self.slept_ms += ms;
    }

    fn now_ms(&mut self) -> u64 {
        self.tricks.now
    }
}
