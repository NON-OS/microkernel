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

//! The modelled registers' behaviour: write-one-to-clear status, refusal,
//! and SCI_EN coming on some sleeps after ACPI_ENABLE.

use crate::events::EventIo;
use crate::regs::Regs;

impl EventIo for Regs {
    fn read(&mut self, port: u16, width: u8) -> Option<u16> {
        assert!(matches!(width, 1 | 2), "width {width}");
        (!self.refused.contains(&port)).then(|| self.get(port))
    }

    fn write(&mut self, port: u16, width: u8, value: u16) -> bool {
        assert!(matches!(width, 1 | 2), "width {width}");
        if self.refused.contains(&port) {
            return false;
        }
        self.writes.push((port, value));
        if self.w1c.contains(&port) {
            let v = self.get(port) & !value;
            self.set(port, v);
        } else {
            self.set(port, value);
        }
        if let Some((smi, enable, _, after)) = self.smi {
            if port == smi && value == enable {
                self.armed_smi = Some(after);
            }
        }
        true
    }

    fn sleep(&mut self, ms: u64) {
        self.slept_ms += ms;
        if let (Some(left), Some((_, _, cnt, _))) = (self.armed_smi, self.smi) {
            if left <= 1 {
                let v = self.get(cnt) | 1;
                self.set(cnt, v);
                self.armed_smi = None;
            } else {
                self.armed_smi = Some(left - 1);
            }
        }
    }
}
