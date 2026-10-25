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

//! Which SLP_TYP pair the kernel's soft-off writes (F-111): its own
//! constant reader's, found at boot in the tables, first; the ACPI
//! capsule's interpreter's, which may compute \_S5 in a method, second.
//! Each value is held to the three bits of the SLP_TYP field (ACPI 6.5,
//! 4.8.3.2.1 and 7.4.2; recalled, unverified): a wider one is refused,
//! never cut, since a cut value names some other sleeping state. None when
//! neither source gives a pair: soft-off is unavailable, and the posture
//! line says so. Pure: `kernel_proofs` mounts it.

pub const SLP_TYP_MAX: u8 = 7;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Source {
    Scan,
    Interpreter,
}

/// The pair soft-off writes, and where it came from.
pub fn choose(scan: Option<(u8, u8)>, interp: Option<(u8, u8)>) -> Option<((u8, u8), Source)> {
    let fits = |p: &(u8, u8)| p.0 <= SLP_TYP_MAX && p.1 <= SLP_TYP_MAX;
    match scan.filter(fits) {
        Some(p) => Some((p, Source::Scan)),
        None => interp.filter(fits).map(|p| (p, Source::Interpreter)),
    }
}

/// The posture line's word for it.
pub fn posture(from: Option<Source>) -> &'static [u8] {
    match from {
        Some(Source::Scan) => b" soft_off=scan",
        Some(Source::Interpreter) => b" soft_off=interpreter",
        None => b" soft_off=none",
    }
}
