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

//! The proofs, one concern per file. `model`, `phy_model` and `memory` are
//! the register window, the PHY behind MDIC, and the rings in host memory
//! the rest are run against.

mod memory;
mod model;
mod phy_model;

mod bring_up_tests;
mod control_tests;
mod decode_fuzz;
mod finish_tests;
mod id_tests;
mod link_tests;
mod log_tests;
mod mdic_live_tests;
mod mdic_word_tests;
mod pci_command_tests;
mod power_up_tests;
mod reset_order_tests;
mod reset_tests;
mod rx_fuzz;
mod rx_queue_tests;
mod rx_wb_tests;
mod semaphore_tests;
mod swfw_tests;
mod timeout_text_tests;
mod tx_encode_tests;
mod tx_queue_tests;
mod tx_ring_tests;
mod tx_wrap_tests;
mod wait_tests;
