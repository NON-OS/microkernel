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

//! The proofs, one concern per file. `memory`, `build`, `model`,
//! `phy_model` and `phy_states` are the grants in host memory, a driver
//! over them, the register window and the PHY the rest are run against.

mod build;
mod config_replay;
mod memory;
mod model;
mod phy_model;
mod phy_states;

mod abort_tests;
mod bring_up_tests;
mod broker_rule_tests;
mod chip_tests;
mod copper_tests;
mod decode_fuzz;
mod finish_tests;
mod hw_cfg_tests;
mod init_register_tests;
mod init_value_tests;
mod led_tests;
mod log_tests;
mod mdio_tests;
mod mode_value_tests;
mod phy_reset_tests;
mod relink_tests;
mod reset_tests;
mod resolve_tests;
mod rx_init_tests;
mod rx_parse_fuzz;
mod rx_parse_tests;
mod rx_queue_tests;
mod stand_down_tests;
mod status_tests;
mod tx_reclaim_tests;
mod tx_ring_tests;
