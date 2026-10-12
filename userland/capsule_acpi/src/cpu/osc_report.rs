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

//! One line on what the processors' _OSC answered, for every processor at
//! once: what was asked, how many granted and what, how many refused.

use super::caps::{word, DECLARED};
use super::state::Cpus;
use crate::sys::log::Line;
use nonos_aml::cpu::ProcessorOsc;

pub fn say_osc(cpus: &Cpus) {
    let (mut granted, mut words, mut refused, mut absent) = (0u64, 0u32, 0u64, 0u64);
    for c in &cpus.list {
        match c.osc {
            ProcessorOsc::Granted(w) => (granted, words) = (granted + 1, words | w),
            ProcessorOsc::Refused(_) => refused += 1,
            ProcessorOsc::Absent => absent += 1,
        }
    }
    let asked = Line::new(b"cpu _OSC asked ").hex(u64::from(word()));
    let asked = DECLARED.iter().fold(asked, |l, c| l.str(b" ").str(c.what.as_bytes()));
    asked
        .str(b": granted ")
        .dec(granted)
        .str(b" (")
        .hex(u64::from(words))
        .str(b"), refused ")
        .dec(refused)
        .str(b", none ")
        .dec(absent)
        .end();
}
