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

extern crate alloc;

use alloc::vec::Vec;

use super::types::Capability;

/// Fold a table of capabilities into a token word.
///
/// Takes the table as an argument rather than reaching for it, so the function
/// is a function of its inputs and the extraction in
/// `verification/extraction/caps` can start from it. The loop is there for the
/// same reason: the iterator adapters this was written with are outside the
/// fragment Aeneas translates, so the version that shipped could not be proven
/// about at all.
#[inline]
pub(crate) fn fold_caps(table: &[Capability], bits: u64) -> u64 {
    let mut acc = bits;
    let mut i = 0;
    while i < table.len() {
        acc |= table[i].bit();
        i += 1;
    }
    acc
}

/// The capabilities of `table` that `bits` grants, in table order.
///
/// The one place the kernel turns a token word back into capabilities. A
/// capability missing from the table it is handed resolves to nothing here and
/// nothing anywhere else notices, which is where `ForeignExec` spent a release.
/// `granting_resolves` in `verification/extraction/lean/NonosExtraction` is the
/// theorem that rules that out, and it is stated about this function.
#[inline]
pub(crate) fn select_caps(table: &[Capability], bits: u64) -> Vec<Capability> {
    let mut out = Vec::new();
    let mut i = 0;
    while i < table.len() {
        let cap = table[i];
        if bits & cap.bit() != 0 {
            out.push(cap);
        }
        i += 1;
    }
    out
}

#[inline]
pub fn caps_to_bits(caps: &[Capability]) -> u64 {
    fold_caps(caps, 0)
}

#[inline]
pub fn bits_to_caps(bits: u64) -> Vec<Capability> {
    select_caps(Capability::all(), bits)
}

#[inline]
pub fn has_capability(bits: u64, cap: Capability) -> bool {
    bits & cap.bit() != 0
}

#[inline]
pub fn add_capability(bits: u64, cap: Capability) -> u64 {
    bits | cap.bit()
}

#[inline]
pub fn remove_capability(bits: u64, cap: Capability) -> u64 {
    bits & !cap.bit()
}

#[inline]
pub fn capability_count(bits: u64) -> usize {
    bits.count_ones() as usize
}
