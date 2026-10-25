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

//! One act on a wipe (L78), no table lock held: start the reset, move a D3hot
//! function back to D0, or look at what a reset came to. Never a wait.

use super::bits_rule::{AF_TP, BME, PM_STATE, TRANSACTIONS_PENDING};
use super::due_rule::{due, Due};
use super::end_rule::End;
use super::method_rule::method;
use super::phase_rule::Phase;
use super::start_rule::{start as rule, Do, Look};
use super::table::Wipe;
use crate::drivers::pci::ConfigSpace;

pub(super) fn act(w: Wipe, now: u64) -> (Wipe, Option<End>) {
    match due(w.phase, now) {
        Due::Start => start(w, now),
        Due::ToD0 => (to_d0(w, now), None),
        Due::Check => super::check::check(w, now),
        Due::Wait => (w, None),
    }
}

fn start(mut w: Wipe, now: u64) -> (Wipe, Option<End>) {
    let Phase::Owed { since } = w.phase else { return (w, None) };
    let (cfg, c) = (ConfigSpace::new(w.address), w.caps);
    let at = |cap: u8, off: u16| u16::from(cap) + off;
    let pmcsr = (c.pm != 0).then(|| cfg.read16(at(c.pm, 4)).ok()).flatten();
    let in_d0 = pmcsr.map_or(true, |p| p & PM_STATE == 0);
    let tp = match (c.pcie, c.af != 0 && c.af_tp) {
        (p, _) if p != 0 => {
            cfg.read16(at(p, 0x0A)).map_or(false, |s| s & TRANSACTIONS_PENDING != 0)
        }
        (_, true) => cfg.read8(at(c.af, 5)).map_or(false, |s| s & AF_TP != 0),
        _ => false,
    };
    let command = cfg.read16(0x04).ok();
    w.bme_off = command.is_some_and(|v| v & BME == 0);
    let m = w.method.or(method(c.offers));
    if m.is_some() && w.saved.is_none() {
        w.saved = command.and_then(|v| super::reset::save(&cfg, c, v));
    }
    if m.is_some() && w.saved.is_some() && w.marker.is_none() {
        w.marker = super::reset::mark(&cfg, c);
    }
    match rule(m, Look { in_d0, tp, marker: w.marker }, since, now) {
        Do::Stay => (w, None),
        Do::Unreset => (w, Some(End::Unreset)),
        Do::Write(m, _) => (super::reset::write(w, m, now), None),
    }
}

fn to_d0(mut w: Wipe, now: u64) -> Wipe {
    let cfg = ConfigSpace::new(w.address);
    let at = u16::from(w.caps.pm) + 4;
    if let Ok(p) = cfg.read16(at) {
        let _ = cfg.write16(at, super::bits_rule::pmcsr_to(p, 0));
    }
    w.phase = Phase::D0 { at: crate::time::now_ns().max(now) };
    w
}
