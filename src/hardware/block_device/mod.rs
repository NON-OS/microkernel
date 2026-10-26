// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

mod announce;
mod backend;
mod booted;
mod capacity;
mod disk;
mod error;
mod fit;
mod flush;
mod found;
mod geometry;
mod gpt;
mod identify;
mod kept;
mod kind;
pub(crate) mod leave;
mod map_ahci;
mod map_nvme;
mod map_usb_msc;
mod map_virtio;
mod mirror;
mod order;
mod pass;
mod placed;
mod read;
mod rows;
mod said;
mod seen;
mod seen_line;
mod select;
pub mod store_copy;
mod types;
mod volume_rule;
mod volume_scope;
mod wait;
mod wait_since;
pub mod wire;
mod write;

pub use backend::Backend;
pub use capacity::capacity;
pub use error::BlockDeviceError;
pub use flush::flush;
pub use geometry::geometry;
pub use gpt::{Extent, Places};
pub use kept::{chosen, places};
pub use kind::Kind;
pub use placed::{plan_lba, read_plan_sector, read_store, write_plan_sector, write_store};
pub use read::read;
pub use select::selected;
pub use types::BlockGeometry;
pub use volume_scope::{disk_caller_admitted, volume_scope, VolumeScope};
pub use write::write;
