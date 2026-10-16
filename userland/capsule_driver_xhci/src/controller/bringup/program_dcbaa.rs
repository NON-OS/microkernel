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
use crate::dma::DmaRegion;
use crate::regs::op::{config_set_max_slots, dcbaap_program};
/// Hand the controller its DCBAA and enable `max_slots` slots.
pub fn program_dcbaa(op_base: u64, dcbaa: &DmaRegion, max_slots: u8) {
    dcbaap_program(op_base, dcbaa.phys());
    config_set_max_slots(op_base, max_slots);
}
