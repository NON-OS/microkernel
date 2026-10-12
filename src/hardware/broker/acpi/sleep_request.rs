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

//! What MkAcpiSleep accepts: soft-off (S5) only, with SLP_TYP values of
//! three bits (ACPI 6.5, 4.8.3.2.1 and 7.4.2). S3 and S4 need a resume
//! path the kernel does not have, and stay refused until it does.

pub const SOFT_OFF: u64 = 5;
const SLP_TYP_MAX: u64 = 7;

pub fn valid(state: u64, a: u64, b: u64) -> bool {
    state == SOFT_OFF && a <= SLP_TYP_MAX && b <= SLP_TYP_MAX
}

/// What the constant reader had, against what the interpreter found.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Before {
    None,
    Same,
    Differed(u8, u8),
}

pub fn compare(before: Option<(u8, u8)>, now: (u8, u8)) -> Before {
    match before {
        None => Before::None,
        Some(b) if b == now => Before::Same,
        Some((a, b)) => Before::Differed(a, b),
    }
}
