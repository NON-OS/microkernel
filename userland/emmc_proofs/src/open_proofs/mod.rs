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

//! The open path's proofs, over the stand-in nonos_libc: Bus Master at open
//! on a host a dead driver left with ADMA running, the refusals, a host still
//! busy after its reset, the regions' address width, the hand-back,
//! Command writes the broker refuses, and a removable slot's open.

mod busy_tests;
mod dma32_tests;
mod give_back_tests;
mod open_refusal_tests;
mod open_tests;
mod refused_write_tests;
mod removable_tests;
