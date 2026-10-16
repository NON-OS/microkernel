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

//! GMBUS's state in the model: the EDID monitor and its read, and the
//! DDC/CI monitor and its transaction.

#[derive(Default)]
pub struct Ddc {
    /// The pin pair the monitor hangs on, and its EDID.
    pub monitor: Option<(u32, Vec<u8>)>,
    /// The read in progress: next byte, end byte.
    pub at: usize,
    pub end: usize,
    pub nak: bool,
    pub active: bool,
    /// A DDC/CI monitor at 0x37 on a pin pair, and the transaction with
    /// it in progress (`gmbus_ddc_hw.rs`).
    pub mccs: Option<(u32, super::mccs::Mccs)>,
    pub ci: Option<super::gmbus_ddc_hw::Ci>,
    pub data: Vec<u32>,
}
