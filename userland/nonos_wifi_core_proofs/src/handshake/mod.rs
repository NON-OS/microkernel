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

//! The four-way and group key handshakes of IEEE Std 802.11-2020, 12.7.6 and
//! 12.7.7, driven by frames an access point would send.

mod forged_m1_tests;
mod four_way_tests;
mod fuzz_tests;
mod group_tests;
mod refusal_tests;
mod reinstall_sim;
mod reinstall_tests;
mod replay_tests;
pub mod sim;
