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

/*
 * The file calls beyond open, read and write.
 */

pub(super) mod create_cloexec;
pub(super) mod create_flags;
pub(super) mod dup3;
pub(super) mod dup3_args;
pub(super) mod falloc;
pub(super) mod fdrange;
pub(super) mod fdup;
pub(super) mod openat2;
pub(super) mod sendfile;
pub(super) mod size;

pub use create_cloexec::{epoll_create1, new_memfd};
pub use dup3::dup3;
pub use falloc::fallocate;
pub use fdrange::close_range;
pub use fdup::dup_from;
pub use openat2::openat2;
pub use sendfile::{copy_file_range, sendfile};
pub use size::{fadvise64, ftruncate, truncate};
