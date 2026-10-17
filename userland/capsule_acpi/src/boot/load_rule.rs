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

//! What a refused definition block means for the rest (F-123). A refused
//! SSDT adds nothing, and the rest load. A refused DSDT, malformed or cut
//! short by the budget (which leaves it half in, since it cannot be taken
//! back out), leaves no namespace worth serving: a half _PRT routes
//! interrupts wrong, a half _PSx powers the wrong device. So none is
//! served: no SSDT is loaded on top, the namespace is dropped for an
//! empty one, which answers no AML-backed request, and the kernel's own
//! MADT routing and scanned \_S5 stand, as on a machine with no AML at
//! all. Pure: the proofs mount it.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Next {
    /// Load the next block.
    Go,
    /// Drop the namespace, load nothing more.
    NoAml,
}

/// After block `index` was refused.
pub fn after_refusal(index: usize) -> Next {
    if index == 0 {
        Next::NoAml
    } else {
        Next::Go
    }
}

use super::Part;

/// Block `index` refused: cut short by the budget, or refused, at `at`.
pub fn refused_line(index: usize, cut: bool, at: u32, out: &mut dyn FnMut(Part)) {
    out(Part::Str(if index == 0 { b"DSDT #" } else { b"SSDT #" }));
    out(Part::Dec(index as u64));
    out(Part::Str(if cut { b" cut short at " } else { b" refused at " }));
    out(Part::Hex(u64::from(at)));
    if after_refusal(index) == Next::NoAml {
        out(Part::Str(b"; no AML is served: the kernel's MADT routing and scanned \\_S5 stand"));
    }
}
