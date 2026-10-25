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

//! Intel hybrid parts: the bit that says a part is one, and the core type
//! CPUID.1AH reports for the core that executes it. Pure, so the host proofs
//! can hold the decoding.

/// CPUID.(EAX=07H, ECX=0):EDX[15], Hybrid. Intel SDM vol. 2A, CPUID.
pub(crate) const LEAF7_EDX_HYBRID: u32 = 1 << 15;
/// The leaf that names the core type, CPUID.1AH. Intel SDM vol. 2A, CPUID.
pub(crate) const LEAF_CORE_TYPE: u32 = 0x1A;
/// CPUID.1AH:EAX[31:24] values: Intel Atom and Intel Core.
const TYPE_ATOM: u32 = 0x20;
const TYPE_CORE: u32 = 0x40;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum CoreType {
    Core,
    Atom,
    Other,
}

/// EAX[23:0] is the native model id and says nothing about the type.
pub(crate) fn core_type(eax_1a: u32) -> CoreType {
    match eax_1a >> 24 {
        TYPE_CORE => CoreType::Core,
        TYPE_ATOM => CoreType::Atom,
        _ => CoreType::Other,
    }
}
