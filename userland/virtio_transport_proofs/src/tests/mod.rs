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

//! The transport against synthetic config spaces, a broker in host memory
//! and a model device.

mod broker;
mod device;
mod space;

mod bringup_tests;
mod caps_tests;
mod features_tests;
mod map_tests;
mod mmio_tests;
mod notify_tests;
mod queue_tests;
mod select_tests;
mod walk_tests;
