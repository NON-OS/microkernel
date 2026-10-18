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

//! What the part saw the driver do, and the faults a test can switch on.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    /// A software reset, with how many grants the driver had given back.
    Reset { grants_back: usize },
    /// A write to CSR `n`.
    Csr(u16, u32),
    /// The transmit process took a setup frame.
    Setup,
}

/// Ways a part can fail the driver, one switch each.
#[derive(Default, Clone, Copy)]
pub struct Faults {
    pub stuck_in_reset: bool,
    pub ignores_setup: bool,
    pub never_starts: bool,
}
