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

//! What HOSTC should hold, and the bring-up that sets it and puts it back.
//! Pure, for the proofs.

use crate::regs::{HOSTC_HST_EN, HOSTC_I2C_EN};

/// The bits the driver changes: the two the broker lets it write
/// (`src/hardware/broker/pci/quirk_bits/writable.rs`).
pub const HOSTC_CHANGED: u8 = HOSTC_HST_EN | HOSTC_I2C_EN;

/// The host enabled and SMBus (not I2C) timing; every other bit, SMI routing
/// among them, as firmware set it.
pub fn hostc_wanted(current: u8) -> u8 {
    (current & !HOSTC_I2C_EN) | HOSTC_HST_EN
}

/// HOSTC in config space: a one-byte read, and a write of the
/// `HOSTC_CHANGED` bits alone, each to the value given.
pub trait HostcCfg {
    fn read(&mut self) -> Option<u8>;
    fn write(&mut self, bits: u8) -> bool;
}

/// The host turned on, then the rest of the bring-up; if that fails, HOSTC
/// is put back as firmware left it, so a host never served is not left on.
pub fn turned_on<C: HostcCfg, T>(
    cfg: &mut C,
    rest: impl FnOnce() -> Result<T, &'static str>,
) -> Result<T, &'static str> {
    let saved = cfg.read().ok_or("smbus: config read refused")?;
    let want = hostc_wanted(saved);
    if want == saved {
        return rest();
    }
    if !cfg.write(want & HOSTC_CHANGED) {
        return Err("smbus: HOSTC write refused");
    }
    let done = rest();
    if done.is_err() {
        let _ = cfg.write(saved & HOSTC_CHANGED);
    }
    done
}
