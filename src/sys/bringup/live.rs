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

//! The hooks in a `nonos-bringup-trace` build: each event said on serial
//! with the time since boot, and kept in the ledger for the summary.

use super::ev::Ev;
use super::ledger::Ledger;
use crate::sys::serial::Line;
use spin::Mutex;

static LEDGER: Mutex<Ledger> = Mutex::new(Ledger::new());

pub fn event(dev: &'static str, ev: Ev) {
    let t = crate::sys::clock::since_boot_ms();
    let mut l = Line::new();
    l.str(b"[BRINGUP] t=").dec(t).str(b" dev=").str(dev.as_bytes());
    l.str(b" ev=").str(ev.word());
    if let Some(on) = ev.on() {
        l.str(b" on=").str(on.as_bytes());
    }
    if let Some(why) = ev.why() {
        l.str(b" why=").str(why.as_bytes());
    }
    l.end();
    LEDGER.lock().note(dev, &ev);
}

/// One line counting every device by its last event, at the end of `stage`.
pub fn summary(stage: &'static str) {
    let t = LEDGER.lock().tally();
    let mut l = Line::new();
    l.str(b"[BRINGUP] summary stage=").str(stage.as_bytes());
    l.str(b" t=").dec(crate::sys::clock::since_boot_ms());
    l.str(b" devices=").dec(u64::from(t.devices)).str(b" up=").dec(u64::from(t.up));
    l.str(b" absent=").dec(u64::from(t.absent)).str(b" waiting=").dec(u64::from(t.waiting));
    l.str(b" failed=").dec(u64::from(t.failed)).str(b" dropped=").dec(u64::from(t.dropped));
    l.end();
}
