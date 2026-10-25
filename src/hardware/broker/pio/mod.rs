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

mod access;
mod bar_window;
mod fixed_line;
mod fixed_view;
mod grant;
mod grant_drain;
mod grant_insert;
mod map;
mod overlap;
mod power_ports;
mod release;
mod types;
mod width;
mod window;

pub use access::{grant_for_caller, read, write};
pub(in crate::hardware::broker) use fixed_view::intx_view;
pub use release::{release_all_for_pid, release_for_device, release_grant};
pub use types::{PioError, PioGrantRequest, PioGrantResult, PioWidth};
