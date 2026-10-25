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

//! The receive half of an associated link: CCMP, delivery, drops and the
//! group key.

mod ccmp_tests;
mod chip_gcmp_tests;
mod chip_hwrx_tests;
mod chip_tests;
mod clear_m3_tests;
mod deliver_tests;
mod drop_tests;
mod forged_rekey;
pub mod frame;
mod fuzz;
mod group_tests;
pub mod join;
mod lie_sweep;
mod rekey;
mod rekey_seed;
