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

//! The bring-up's console lines: one per step that went wrong, with the
//! values that decided it, and one for the controller and namespace served.

mod failure;
mod namespace;
mod refused;
mod serving;

pub use failure::{attempt_failed, step_failed, unsupported};
pub use namespace::{geometry, list_refused, no_namespace};
pub use refused::refused;
pub use serving::serving;
