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

//! The Type-C half of the power wire, after the battery and AC fields: the
//! number of ports served (0 with no UCSI, at most `MAX_PORTS`), then four
//! bytes a port: flags (1 attached, 2 sourcing power, 4 a charger: a
//! partner this port sinks from, 8 can source, 16 can sink, 32 status
//! unknown, 64 the session stopped reading), the power operation mode,
//! the partner type and the battery charging status, as UCSI gives them.
//! Then for the same ports in order, a charger's offers and what was
//! taken: the count of source PDOs u8 (0 when none were read) and each
//! PDO u32 as the charger sent it, then the request's position u8 and
//! its mV, mA and mW u32 (`nonos_ucsi::contract`; 0 where the request
//! does not state one, all 0 with no contract).

use crate::boot::Acpi;
use alloc::vec::Vec;
use nonos_ucsi::contract::contract;
use nonos_ucsi::session::State;

pub const MAX_PORTS: usize = 8;

pub fn append(acpi: &Acpi, v: &mut Vec<u8>) {
    let Some(t) = &acpi.typec else { return v.push(0) };
    let stopped = if t.session.state == State::Failed { 64 } else { 0 };
    let ports = &t.session.ports[..t.session.ports.len().min(MAX_PORTS)];
    v.push(ports.len() as u8);
    for p in ports {
        let cap = p.capability.unwrap_or_default();
        let can = u8::from(cap.provider) << 3 | u8::from(cap.consumer) << 4;
        let Some(s) = p.status else {
            v.extend_from_slice(&[can | 32 | stopped, 0, 0, 0]);
            continue;
        };
        let now = u8::from(s.connected) | u8::from(s.provider) << 1 | u8::from(s.charger()) << 2;
        v.extend_from_slice(&[now | can | stopped, s.power_mode, s.partner_type, s.charging]);
    }
    for p in ports {
        let offers = p.offered.list();
        v.push(offers.len() as u8);
        offers.iter().for_each(|w| v.extend_from_slice(&w.to_le_bytes()));
        let rdo = p.status.map_or(0, |s| s.rdo);
        let c = contract(rdo, offers).unwrap_or_default();
        v.push(c.position);
        [c.mv, c.ma, c.mw].iter().for_each(|x| v.extend_from_slice(&x.to_le_bytes()));
    }
}
