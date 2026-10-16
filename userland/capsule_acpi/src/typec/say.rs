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

//! The `[ACPI] ucsi` lines: the session's start, each port that changed,
//! and the session's own state. A port's line carries what the power wire
//! serves, nothing that identifies the partner.

use super::state::TypeC;
use crate::sys::log::Line;
use nonos_ucsi::contract::contract;
use nonos_ucsi::pdo::{max_mw, pdo};
use nonos_ucsi::session::State;
use nonos_ucsi::start::Port;

pub fn state(what: &[u8]) {
    Line::new(b"ucsi: ").str(what).end();
}

pub fn transition(s: State) {
    state(match s {
        State::Running => b"running",
        State::Recovering { failures: 0, .. } => b"the PPM stuck; resetting it",
        State::Recovering { .. } => b"reset failed; trying again later",
        State::Failed => b"three resets failed; Type-C state no longer read",
    });
}

pub fn changed(n: usize, p: &Port) {
    let l = Line::new(b"ucsi port ").dec(n as u64);
    let Some(s) = p.status else { return l.str(b": status unknown").end() };
    if !s.connected {
        return l.str(b": nothing attached").end();
    }
    let role: &[u8] = if s.provider { b" sourcing" } else { b" sinking" };
    let l = l.str(b": attached").str(role).str(b" mode=").dec(u64::from(s.power_mode));
    let l = l.str(b" partner=").dec(u64::from(s.partner_type));
    let l = l.str(if s.charger() { b" charger" } else { b"" });
    let best = p.offered.list().iter().map(|w| max_mw(pdo(*w))).max();
    let Some(best) = best else { return l.end() };
    let took = contract(s.rdo, p.offered.list()).map_or(0, |c| c.mw);
    l.str(b" offers up to ").dec(best.into()).str(b" mW, took ").dec(took.into()).str(b" mW").end();
}

pub fn started(t: &TypeC) {
    let l = Line::new(b"ucsi: ").dec(t.session.ports.len() as u64).str(b" ports, mailbox ");
    l.hex(t.mailbox.0).str(b"+").hex(t.mailbox.1).end();
    for (i, p) in t.session.ports.iter().enumerate() {
        changed(i + 1, p);
    }
}
