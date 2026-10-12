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

//! What NONOS tells each processor's _OSC it implements: one row per
//! kernel capability, and only for code the kernel has. The bits are the
//! processor vendor's (Intel's processor vendor-specific ACPI interface),
//! not ACPI's. The kernel has no P-state, C-state or CPPC control yet, so
//! the table is empty and the word is 0; firmware still hears that an
//! operating system that knows processor _OSC is present. The unit that
//! lands that control adds its row here, citing the vendor's document.

pub struct Capability {
    /// The kernel code that makes the claim true.
    pub what: &'static str,
    pub bit: u32,
}

pub const DECLARED: &[Capability] = &[];

pub fn word() -> u32 {
    DECLARED.iter().fold(0, |w, c| w | 1 << c.bit)
}
