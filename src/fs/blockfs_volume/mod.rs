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

mod create;
mod error;
mod format_volume;
mod hex;
mod import;
mod import_one;
mod import_record;
mod import_stream;
mod imported;
mod key_to_array;
mod list;
mod mount_volume;
mod open_machine;
mod plan;
mod plan_read;
mod plan_types;
mod read;
mod read_all;
mod read_at;
mod remove;
mod ring_blank;
mod stat;
mod state;
mod write;
mod write_or_create;

pub use create::create;
pub use error::VolumeError;
pub use format_volume::format_volume;
pub use import::import;
pub use imported::Imported;
pub use list::list;
pub use mount_volume::mount_volume;
pub use open_machine::open_machine_volume;
pub use plan::parse_plan;
pub use plan_types::{Plan, PlanError, DATA_FLOOR, MAX_IMPORTS, PLAN_LBA};
pub use read::read;
pub use read_all::{read_all, WHOLE_READ_MAX};
pub use read_at::read_at;
pub use remove::remove;
pub use stat::stat;
pub use write::write;
pub use write_or_create::write_or_create;
