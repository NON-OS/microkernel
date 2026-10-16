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

//! The proofs, by what they hold: the pure modules against Linux and the
//! SD specification, then the flows over the modelled reader.

mod bipr_tests;
mod chip_tests;
mod clock_tests;
mod csd_tests;
mod encode_tests;
mod line_tests;
mod response_tests;
mod sd_tests;

mod admits_tests;
mod away_tests;
mod card_flow_tests;
mod card_rw_tests;
mod claim_tests;
mod client_tests;
mod data_fail_tests;
mod deadline_tests;
mod fuzz_tests;
mod hostile_tests;
mod identity_tests;
mod layout_tests;
mod medium_tests;
mod protect_tests;
mod recover_tests;
mod registers_tests;
mod release_tests;
mod removal_tests;
mod rig;
mod rng;
mod serve_rw_tests;
mod service_rig;
mod setup_tests;
mod size_tests;
mod stall_tests;
mod stop_seen_tests;
mod swap_tests;
mod teardown_tests;
mod wait_tests;
mod wipe_tests;
mod wire_tests;
mod worst_case_tests;
mod write_wait_tests;
