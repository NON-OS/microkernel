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

//! The requests the driver makes, each carried out in one write of its
//! bus address to the request port.

mod events;
mod guest;
mod mouse;
mod page;

pub use events::acknowledge;
pub use guest::HostVersion;
pub use guest::{host_version, report_guest_info, set_event_filter, set_no_capabilities};
pub use mouse::{get_mouse, set_mouse};
pub use page::Page;
