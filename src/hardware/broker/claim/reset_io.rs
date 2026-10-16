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

//! ResetIo over a Function's config space.

use super::reset_function::Function;
use super::reset_rule::{af_flr, pcie_flr, Flr, ResetIo};

use super::reset_restore::INITIATE_FLR;

const TRANSACTIONS_PENDING: u16 = 1 << 5;

impl ResetIo for Function {
    fn flr(&mut self) -> Option<Flr> {
        if self.cfg.header_type().ok()? & 0x7F != 0 {
            return None;
        }
        if let Some(p) = self.pcie {
            if pcie_flr(self.cfg.read32(p + 4).ok()?) {
                return Some(Flr::Pcie);
            }
        }
        af_flr(self.cfg.read8(self.af? + 3).ok()?).then_some(Flr::Af)
    }
    fn pending(&mut self, how: Flr) -> bool {
        let status = match (how, self.pcie, self.af) {
            (Flr::Pcie, Some(p), _) => self.cfg.read16(p + 0x0A).map(|s| s & TRANSACTIONS_PENDING),
            (Flr::Af, _, Some(a)) => self.cfg.read8(a + 5).map(|s| u16::from(s & 1)),
            _ => return false,
        };
        status.map_or(false, |s| s != 0)
    }
    fn save(&mut self) {
        for (i, dword) in self.header.iter_mut().enumerate() {
            *dword = self.cfg.read32(i as u16 * 4).unwrap_or(0);
        }
        if let Some(p) = self.pcie {
            let control = self.cfg.read16(p + 8).unwrap_or(0);
            self.control = (control, self.cfg.read16(p + 0x28).unwrap_or(0));
        }
    }
    fn initiate(&mut self, how: Flr) {
        let _ = match (how, self.pcie, self.af) {
            (Flr::Pcie, Some(p), _) => self.cfg.write16(p + 8, self.control.0 | INITIATE_FLR),
            (Flr::Af, _, Some(a)) => self.cfg.write8(a + 4, 1),
            _ => Ok(()),
        };
    }
    fn ready(&mut self) -> bool {
        matches!(self.cfg.vendor_id(), Ok(v) if v != 0xFFFF && v != 0x0001)
    }
    fn restore(&mut self) {
        self.restore_header();
    }
    fn now_ms(&mut self) -> u64 {
        crate::time::timestamp_millis()
    }
    fn pause_ms(&mut self, ms: u64) {
        super::reset_restore::pause_ms(ms, self.sleeper);
    }
}
