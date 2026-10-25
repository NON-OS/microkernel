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

//! The codec's registers as QEMU's STAC9700 holds them: muted out of
//! reset, the vendor id read-only, the powerdown status bits the codec's
//! own, the front DAC fixed at 48 kHz unless variable rate is on. Every
//! mixer access gives the codec semaphore back.

use super::codec::{EXT_CTRL, FRONT_RATE, MASTER, PCM_OUT, POWERDOWN, VENDOR1, VENDOR2};
use super::ich::Ich;

impl Ich {
    pub fn nam_read(&mut self, offset: u16, width: u8) -> u32 {
        assert_eq!((width, offset & 1), (2, 0), "the codec is read a register at a time");
        self.cas = false;
        if self.opt.decode_off {
            return 0xFFFF;
        }
        let v = self.mixer[offset as usize / 2];
        let ready = if self.opt.powered { 0xF } else { 0 };
        (if offset as usize / 2 == POWERDOWN { (v & !0xF) | ready } else { v }) as u32
    }

    pub fn nam_write(&mut self, offset: u16, width: u8, value: u32) {
        assert_eq!((width, offset & 1), (2, 0), "the codec is written a register at a time");
        self.writes.push((super::NAM_GRANT, offset, value));
        self.cas = false;
        if self.opt.decode_off {
            return;
        }
        let reg = offset as usize / 2;
        let v = value as u16;
        match reg {
            0 => self.reset_codec(),
            VENDOR1 | VENDOR2 | 0x14 => {}
            EXT_CTRL => {
                self.mixer[reg] = v;
                if v & 1 == 0 {
                    self.mixer[FRONT_RATE] = 48_000;
                }
            }
            _ => self.mixer[reg] = v,
        }
    }

    /// Whether what the engine plays reaches the line out.
    pub fn open(&self) -> bool {
        self.mixer[MASTER] & 0x8000 == 0 && self.mixer[PCM_OUT] & 0x8000 == 0
    }
}
