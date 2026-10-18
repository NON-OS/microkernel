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

//! What the capsule says of its bring-up (F-123): the _REG faults per
//! space, \_PIC(1)'s outcome, and the _INI walk's counts, so a phase cut
//! short by the budget is said as faulted, never as done. Pure: the
//! proofs mount it.

use super::Part;

/// None when no _REG faulted in `space`.
pub fn reg_line(failed: usize, space: u8, out: &mut dyn FnMut(Part)) {
    if failed == 0 {
        return;
    }
    out(Part::Str(b"_REG failed in "));
    out(Part::Dec(failed as u64));
    out(Part::Str(b" scopes, space "));
    out(Part::Dec(u64::from(space)));
}

/// `pic`: None with no \_PIC, else where it faulted, if it did.
pub fn pic_line(pic: Option<Result<(), u32>>, out: &mut dyn FnMut(Part)) {
    match pic {
        Some(Err(at)) => {
            out(Part::Str(b"\\_PIC(1) faulted at "));
            out(Part::Hex(u64::from(at)));
        }
        Some(Ok(())) => out(Part::Str(b"\\_PIC(1): APIC mode")),
        None => out(Part::Str(b"no \\_PIC")),
    }
}

/// _INI's: how many ran, how many of those failed, how many devices were
/// absent, and how many _STA failed.
pub fn ini_line(counts: [usize; 4], out: &mut dyn FnMut(Part)) {
    let words: [&'static [u8]; 4] = [b"_INI ran ", b", failed ", b"; absent ", b"; _STA failed "];
    for (w, n) in words.iter().zip(counts) {
        out(Part::Str(w));
        out(Part::Dec(n as u64));
    }
}
