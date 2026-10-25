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

//! The transfer engine, the probe and the handler against the modelled core.

mod acpi;
mod bounce_tests;
mod busy_tests;
mod bystanders;
mod client;
mod declared;
mod doorbell_tests;
mod fifo_tests;
mod fixture;
mod handler_tests;
mod host_by_address_tests;
mod host_hostile_tests;
mod host_id_tests;
mod host_name_tests;
mod host_tests;
mod info_tests;
mod kept;
mod last_byte_tests;
mod last_resort_tests;
mod listed;
mod lpss_tests;
mod multi_sku;
mod nack_tests;
mod named_hostile_tests;
mod named_tests;
mod named_wire_tests;
mod narrow_tests;
mod order_tests;
mod pace_tests;
mod probe_tests;
mod reach_tests;
mod read_tests;
mod record_fields_tests;
mod records;
mod recover_tests;
mod remap_tests;
mod reread_tests;
mod retained_tests;
mod settle_tests;
mod sweep_neighbour_tests;
mod sweep_scope_tests;
mod sweep_tests;
mod twice_tests;
mod write_tests;
