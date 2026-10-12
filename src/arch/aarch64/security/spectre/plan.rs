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

//! What one CPU does about each speculation issue, and why.

pub use super::branch::{Bhb, V2};

/// Who said the CPU is not affected.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Why {
    /// An ID register field.
    Id,
    /// The firmware, through SMCCC_ARCH_FEATURES.
    Firmware,
    /// Arm's published list for this MIDR.
    Core,
}

/// Speculative store bypass (variant 4).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ssbd {
    NotAffected(Why),
    /// PSTATE.SSBS cleared, and SCTLR_EL1.DSSBS clear so every exception
    /// entry clears it again.
    Ssbs,
    /// SMCCC_ARCH_WORKAROUND_2 called once on this CPU to turn it on.
    Firmware,
    Vulnerable,
}

/// Meltdown (variant 3): EL0 reading the kernel's mappings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Meltdown {
    NotAffected(Why),
    /// EL0 accesses to TTBR1's half fault without walking (C-20).
    E0pd,
    /// Affected, and the kernel stays mapped while EL0 runs.
    Exposed,
    /// Not an Arm core, and its ID registers do not say.
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Plan {
    pub v2: V2,
    pub bhb: Bhb,
    pub ssbd: Ssbd,
    pub meltdown: Meltdown,
}
