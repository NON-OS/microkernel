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

//! When to read: everything at start and every minute; sooner while a
//! reading is owed (`retry`); a battery's state on its Notify 0x80 and its
//! information on 0x81 (ACPI 6.5, 10.2.1), an adapter on its 0x80 (10.3).

use super::alarm::set_trip;
use super::bst_told::tell;
use super::levels::check;
use super::read::{read_adapter, read_info, read_status};
use super::report::send;
use super::state::Power;
use crate::boot::evaluator;
use crate::host::CapsuleHost;
use crate::sys::clock::now_ms;
use nonos_aml::load::Aml;
use nonos_aml::namespace::Id;

const EVERY_MS: u64 = 60_000;
/// While a battery is critical, often enough that the kernel's 30 s grace
/// is measured, not missed.
const CRITICAL_EVERY_MS: u64 = 10_000;

pub fn all(p: &mut Power, aml: &mut Aml, host: &mut CapsuleHost) {
    for b in p.batteries.iter_mut() {
        let sta = read_info(b, aml, &mut evaluator(host));
        set_trip(b, aml, host);
        read_status(b, aml, &mut evaluator(host));
        tell(b);
        super::trace_say::battery(aml, b, &sta);
    }
    for a in p.adapters.iter_mut() {
        read_adapter(a, aml, &mut evaluator(host));
    }
    super::trace_say::adapters(aml, p);
    p.read_ms = now_ms();
    p.retry.after(super::retry::wanted(p), p.read_ms);
    send(p);
    check(p);
}

pub fn tick(p: &mut Power, aml: &mut Aml, host: &mut CapsuleHost) {
    let every = if p.critical { CRITICAL_EVERY_MS } else { EVERY_MS };
    let now = now_ms();
    if now.saturating_sub(p.read_ms) >= every || p.retry.due(now) {
        all(p, aml, host);
    }
}

/// Whether the notification was for one of these devices.
pub fn notified(p: &mut Power, aml: &mut Aml, host: &mut CapsuleHost, id: Id, value: u8) -> bool {
    if let Some(b) = p.batteries.iter_mut().find(|b| b.id == id) {
        if value == 0x81 || b.info.is_none() {
            let sta = read_info(b, aml, &mut evaluator(host));
            set_trip(b, aml, host);
            super::trace_say::battery(aml, b, &sta);
        }
        read_status(b, aml, &mut evaluator(host));
        tell(b);
    } else if let Some(a) = p.adapters.iter_mut().find(|a| a.0 == id) {
        read_adapter(a, aml, &mut evaluator(host));
    } else {
        return false;
    }
    send(p);
    check(p);
    true
}
