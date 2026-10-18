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

//! One line per CPU whose posture differs from the boot CPU's, and the boot
//! CPU's always, so a log says what each core was exposed to and what the
//! kernel did, alongside x86's speculation lines.

use super::plan::{Bhb, Plan};
use super::words;
use crate::sys::serial::Line;

pub(super) fn line(plan: &Plan, smccc: Option<u32>, cpu_part: u16) {
    let mut line = Line::new();
    line.str(b"[SPECTRE] part ").hex(u64::from(cpu_part));
    line.str(b"; v2 ").str(words::v2(plan.v2));
    line.str(b"; BHB ").str(words::bhb(plan.bhb));
    if let Bhb::Loop(k) = plan.bhb {
        line.dec(u64::from(k));
    }
    line.str(b"; SSBD ").str(words::ssbd(plan.ssbd));
    line.str(b"; Meltdown ").str(words::meltdown(plan.meltdown));
    line.str(b"; SMCCC ");
    match smccc {
        Some(v) => line.dec(u64::from(v >> 16)).str(b".").dec(u64::from(v & 0xFFFF)),
        None => line.str(b"1.0 or none"),
    };
    line.end();
}
