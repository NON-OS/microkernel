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

//! Device power (ACPI 6.5, chapter 7): shared power resources, counted
//! and switched in ResourceOrder, and moves between every D-state.

mod dstate;
mod method;
mod order;
mod pool;
mod psc;
mod reveal;
mod rules;
mod state;
mod transition;

pub use dstate::{to_d0, to_d3};
pub use order::{in_order, resource_order};
pub use pool::PowerPool;
pub use psc::{power_managed, psc};
pub use reveal::{reveal, settle, Revealed};
pub use rules::MoveError;
pub use state::DState;
pub use transition::DevicePower;
