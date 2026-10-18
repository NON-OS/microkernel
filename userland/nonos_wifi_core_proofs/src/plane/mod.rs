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

//! E-58: the data plane the network stack drives, across the link's
//! transitions. No data frame leaves before the keys are in
//! (`before_keys_tests`); none is delivered, and none leaves, once teardown
//! has begun — a deauthentication, a loss, or the chip's restart
//! (`after_teardown_tests`); and over a seeded interleaving of transmit,
//! receive, join and teardown in every order, a non-EAPOL data frame leaves
//! only while the link reports up (`order_sweep`).

mod after_teardown_tests;
mod before_keys_tests;
mod drive;
mod order_sweep;
mod restart_tests;
mod rig;
