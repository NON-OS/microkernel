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
//! with FileSystem, and brings a file in, or keys the volume with a
//! passphrase, only with StoreWrite as well. A file fed in chunk by chunk
//! takes StreamImport alone, which grants no read.

mod errno;
mod feed;
mod feed_begin;
mod feed_errno;
mod import;
mod name;
mod passphrase;
mod plan_errno;
mod read;
mod read_bounce;
mod read_peer;
mod remove;
mod stat;

pub use feed::sys_data_feed;
pub use feed_begin::sys_data_feed_begin;
pub use import::sys_data_import;
pub use passphrase::sys_data_passphrase;
pub use read::sys_data_read;
pub use remove::sys_data_remove;
pub use stat::sys_data_stat;
