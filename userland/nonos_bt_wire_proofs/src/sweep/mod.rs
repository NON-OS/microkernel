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
//! E-50: seeded sweeps over the wire decoders. Each input is built sound,
//! with what it holds known from the building; then one lie per rule.

mod acl_tests;
mod adv;
mod att_tests;
mod event_tests;
mod frame_tests;
mod join_tests;
mod list_tests;
mod rng;
mod signal_mutant;
mod signal_tests;
