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

//! Proofs of the Realtek setup: the capsule's `rtl` compiled unchanged,
//! against a model ROM that takes a download as the chip does.

#[allow(unused)]
#[path = "../../../capsule_driver_bt_usb/src/rtl/mod.rs"]
pub mod rtl;

mod epatch_tests;
mod fixture;
mod fixture_image;
mod fixture_setup;
mod frag_tests;
mod image_refuse_tests;
mod image_tests;
mod mem;
mod pin_tests;
mod project_tests;
mod real_pin_tests;
mod rom_sim;
mod rom_sim_transport;
mod setup_answer_tests;
mod setup_refuse_tests;
mod setup_tests;
mod table_row_tests;
mod table_tests;
