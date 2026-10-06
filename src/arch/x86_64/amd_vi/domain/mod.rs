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

//! Capsule domains on AMD-Vi, behind the same broker calls as VT-d.

mod attach;
mod lifetime;
mod map;
mod range;
mod unmap;
mod walk;

pub use attach::{attach, detach, device_id};
pub(super) use attach::pass;
pub use lifetime::{create_domain, destroy_domain, MAX_DOMAINS};
pub use map::map_range;
pub use unmap::unmap_range;
