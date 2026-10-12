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
//! The masked form of `MkPciConfigWrite` (L37). Pure, so kernel_proofs
//! holds it by path.
//!
//! The plain form ORs a Command value that holds only writable bits onto the
//! register, which is what every driver's claim relies on, but it means a
//! driver can never clear Bus Master or Interrupt Disable: writing the
//! register without the bit asks for nothing and reports success. A caller
//! that sets bits 16 to 31 of the offset argument names the bits it changes
//! instead, and each named bit takes the value given, set or clear.

/// Where the change mask sits in the call's offset argument.
pub(crate) const MASK_SHIFT: u32 = 16;

/// Configuration space ends at 4 KiB, so bits 12 to 15 name nothing.
const REGISTER_BITS: u32 = 0x0FFF;

/// The register the call names and its change mask (0 for the plain form),
/// or `None` when bits 12 to 15 are set.
pub(crate) const fn split(arg: u32) -> Option<(u16, u16)> {
    if arg & 0xF000 != 0 {
        return None;
    }
    Some(((arg & REGISTER_BITS) as u16, (arg >> MASK_SHIFT) as u16))
}

/*
 * A value bit outside the mask is refused rather than dropped: the caller
 * believes it set that bit, and a write that quietly did less is the defect
 * this form exists to end.
 */
/// The register after a masked write: the named bits as `value` has them,
/// every other bit as it is.
pub(crate) const fn merge(value: u16, mask: u16, current: u16) -> Option<u16> {
    if value & !mask != 0 {
        return None;
    }
    Some((current & !mask) | value)
}
