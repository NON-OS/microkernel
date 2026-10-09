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

//! dup3's, epoll_create1's and memfd_create's flags; epoll's rules, apart
//! from any descriptor; the span a punched hole zeroes; the mounts a hard
//! link may not cross; the lock table's splitting and ceiling; the record
//! /proc keeps of each image; and the strings and paths read out of a
//! guest, with the errno each refusal is.

#[path = "../../../../capsule_linux/src/linux/file/cstr.rs"]
pub mod cstr;
#[path = "../../../../capsule_linux/src/linux/file/calls/create_flags.rs"]
pub mod create_flags;
#[path = "../../../../capsule_linux/src/linux/file/calls/dup3_args.rs"]
pub mod dup3_args;
#[path = "../../../../capsule_linux/src/linux/file/epoll_rules.rs"]
pub mod epoll_rules;
pub mod exe;
pub mod link;
pub mod lock;
#[path = "../../../../capsule_linux/src/linux/file/made/proc/mounts/table.rs"]
pub mod mounts;
#[path = "../../../../capsule_linux/src/linux/file/path.rs"]
pub mod path;
#[path = "../../../../capsule_linux/src/linux/file/calls/falloc/punch.rs"]
pub mod punch;
pub mod store;

pub use crate::resolve::key;
pub use store::{read as store_read, write as store_write};
