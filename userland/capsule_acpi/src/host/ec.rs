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
//! the EC's _GLK asks for it; none once the EC has stalled or flooded.

use super::lock_hold::{give, take};
use super::spaces::ec_ok;
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
    let (ports, glk) = h.ec.filter(|_| h.ec_health.usable())?;
    let addr = u8::try_from(addr).ok().filter(|_| ec_ok(addr, width))?;
    let r = locked(h, glk, || Some(ec::read(&ports, &mut PortIo, addr)))?;
    heed(h, &r);
    r.ok().map(u64::from)
}

pub(super) fn write(h: &mut CapsuleHost, addr: u64, width: u32, v: u64) -> bool {
    let Some((ports, glk)) = h.ec.filter(|_| h.ec_health.usable()) else { return false };
    let Some(addr) = u8::try_from(addr).ok().filter(|_| ec_ok(addr, width)) else { return false };
    let Some(r) = locked(h, glk, || Some(ec::write(&ports, &mut PortIo, addr, v as u8))) else {
        return false;
    };
    heed(h, &r);
    r.is_ok()
}

/// One access's answer counted; the line said once if it turned the EC off.
fn heed<T>(h: &mut CapsuleHost, r: &Result<T, ec::EcError>) {
    if h.ec_health.note(r) {
        crate::ec::say_off(h.ec_health.off);
    }
}
