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

//! A hostile DisplayPort sink, seeded: capabilities that lie, link status
//! that never settles or flips, AUX replies that DEFER forever, NACK, come
//! short or run past what was asked. Every path ends within its retry and
//! try counts, and a failure leaves the port down.

mod aux_tests;
mod caps_tests;
mod fixed_tests;
mod profile;
mod rng;
mod sink;
mod sink_dpcd;
mod sink_link;
mod state_tests;
mod train_rig;
mod train_tests;
