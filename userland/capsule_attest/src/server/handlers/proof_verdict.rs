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

//! Settling one invariant against the live machine.

use crate::state::live::Snapshot;
use crate::state::Probe;

/// Checked, and it holds.
pub const V_HOLDS: u8 = 0;
/// Checked, and it does not.
pub const V_BROKEN: u8 = 1;
/// Not settled here. Either the claim is not a runtime property, or the kernel
/// would not answer. Never a pass: six claims served as prose were once
/// indistinguishable from six claims that had been verified.
pub const V_UNCHECKED: u8 = 2;

// `found` counts what the claim forbids, so zero is the passing value for every
// probe that has one. A caller comparing verdicts across records never has to
// know which direction a particular claim runs in.
pub fn evaluate(probe: Probe, snap: Option<&Snapshot>) -> (u8, u32, u32) {
    let Some(snap) = snap else {
        return (V_UNCHECKED, 0, 0);
    };
    let of = snap.count as u32;
    match probe {
        Probe::NotAtRuntime => (V_UNCHECKED, 0, of),
        Probe::NoneHold(mask) => verdict(snap.holders(mask), of),
        Probe::OnlyInit(mask) => verdict(snap.holders_beyond_init(mask), of),
        Probe::AllMasked => verdict(snap.unmasked(), of),
    }
}

fn verdict(found: u32, of: u32) -> (u8, u32, u32) {
    (if found == 0 { V_HOLDS } else { V_BROKEN }, found, of)
}

pub fn write_field(out: &mut [u8], bytes: &[u8]) -> usize {
    out[0..4].copy_from_slice(&(bytes.len() as u32).to_le_bytes());
    out[4..4 + bytes.len()].copy_from_slice(bytes);
    4 + bytes.len()
}
