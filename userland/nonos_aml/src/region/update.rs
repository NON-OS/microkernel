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

//! UpdateRule (section 19.6.48, FieldFlags bits 5-6): what a write puts in
//! the bits of an access unit that are not the field's.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UpdateRule {
    /// The other bits are read first and written back unchanged.
    Preserve,
    WriteAsOnes,
    WriteAsZeros,
}

impl UpdateRule {
    /// From FieldFlags; the reserved value 3 is read as Preserve, the one
    /// rule that cannot change a bit the field does not own.
    pub fn from_flags(flags: u8) -> Self {
        match (flags >> 5) & 3 {
            1 => UpdateRule::WriteAsOnes,
            2 => UpdateRule::WriteAsZeros,
            _ => UpdateRule::Preserve,
        }
    }
}

/// The unit to write: `value`'s low `bits` bits at `shift`, the rest from
/// `old` (Preserve) or set by the rule.
pub fn merge(old: u64, value: u64, shift: u32, bits: u32, rule: UpdateRule) -> u64 {
    let field = if bits >= 64 { u64::MAX } else { ((1u64 << bits) - 1) << shift };
    let rest = match rule {
        UpdateRule::Preserve => old,
        UpdateRule::WriteAsOnes => u64::MAX,
        UpdateRule::WriteAsZeros => 0,
    };
    (rest & !field) | ((value << shift) & field)
}
