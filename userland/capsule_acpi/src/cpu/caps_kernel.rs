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

//! The kernel's processor-control features a processor _OSC bit can
//! promise (`caps`), and which of them the kernel has today. Pure.

/// The kernel's processor-control features, each false until it exists.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Kernel {
    /// Sets a _PSS state through _PCT's model-specific registers.
    pub pstates: bool,
    /// Enters a _CST state by I/O read followed by halt.
    pub cstates_io: bool,
    /// Enters a _CST state through the processor's native instruction.
    pub cstates_native: bool,
    /// Sets a _TSS state through _PTC's model-specific registers.
    pub tstates: bool,
    /// Honours _PSD, _TSD and _CSD coordination across processors.
    pub coordination: bool,
    /// Drives _CPC (CPPC) performance requests.
    pub cppc: bool,
}

/// What is in the kernel today: nothing.
pub const KERNEL: Kernel = Kernel {
    pstates: false,
    cstates_io: false,
    cstates_native: false,
    tstates: false,
    coordination: false,
    cppc: false,
};
