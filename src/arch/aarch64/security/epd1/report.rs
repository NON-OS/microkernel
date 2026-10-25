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

//! The trampoline's boot line, once every CPU has started.

use core::sync::atomic::Ordering;

use super::activate::REFUSED;
use super::state::{EXPOSED, SWITCHED};
use crate::sys::serial::Line;

pub fn report() {
    let exposed = EXPOSED.load(Ordering::Acquire);
    let on = SWITCHED.load(Ordering::Acquire);
    let mut line = Line::new();
    line.str(b"[KPTI] ");
    if exposed == 0 {
        line.str(b"no CPU needs the EPD1 trampoline").end();
        return;
    }
    line.str(b"EPD1 trampoline on ").dec(on.into()).str(b" of ").dec(exposed.into());
    line.str(b" exposed CPUs");
    if let Some(reason) = REFUSED.get() {
        line.str(b", not built: ").str(reason.as_bytes());
    }
    line.end();
}
