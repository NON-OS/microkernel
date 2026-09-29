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

//! The data volume, reached by name: a verified import, a size, and a read
//! of any range. The volume is the machine's; a capsule reaches it only
//! with FileSystem, and brings a file in only with StoreWrite as well.

mod errno;
mod import;
mod name;
mod read;
mod stat;

pub use import::sys_data_import;
pub use read::sys_data_read;
pub use stat::sys_data_stat;
