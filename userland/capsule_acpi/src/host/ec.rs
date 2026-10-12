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

//! EmbeddedControl fields: one byte per access (the interpreter reads EC
//! fields by bytes), through the EC's ports, inside the Global Lock when
//! the EC's _GLK asks for it.

use super::lock_hold::{give, take};
use super::state::CapsuleHost;
use crate::events_io::PortIo;
use nonos_aml::ec;

fn locked<T>(h: &mut CapsuleHost, glk: bool, f: impl FnOnce() -> Option<T>) -> Option<T> {
    let mine = glk && !h.lock_held;
    if mine && !take(h) {
        return None;
    }
    let r = f();
    if mine {
        give(h);
    }
    r
}

pub(super) fn read(h: &mut CapsuleHost, addr: u64, width: u32) -> Option<u64> {
    let (ports, glk) = h.ec?;
    let addr = u8::try_from(addr).ok().filter(|_| width == 1)?;
    locked(h, glk, || ec::read(&ports, &mut PortIo, addr).ok().map(u64::from))
}

pub(super) fn write(h: &mut CapsuleHost, addr: u64, width: u32, v: u64) -> bool {
    let Some((ports, glk)) = h.ec else { return false };
    let Ok(addr) = u8::try_from(addr) else { return false };
    width == 1 && locked(h, glk, || ec::write(&ports, &mut PortIo, addr, v as u8).ok()).is_some()
}
