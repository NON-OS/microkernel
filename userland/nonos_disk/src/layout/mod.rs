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

//! Where everything sits on a disk of a given size: the three regions the
//! kernel reads at fixed sectors, the data volume after them, and the EFI
//! system partition at the end.

mod at;
mod extent;
mod plan;
mod region;
mod sizes;

pub use extent::Extent;
pub use plan::Layout;
pub use region::Region;
pub use sizes::{DATA_MIN_SECTORS, ESP_SECTORS, MIN_DISK_SECTORS};
