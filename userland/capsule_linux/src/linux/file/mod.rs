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

/* The filesystem a guest sees. */

mod at;
mod calls;
mod clamp;
pub(super) mod close;
mod cstr;
mod dev;
mod dev_io;
mod dev_metrics;
pub mod dev_metrics_parse;
mod dev_stat;
mod dir;
mod dir_children;
mod dirent;
mod dirents;
mod dirops;
mod epoll;
mod epoll_arm;
mod epoll_wait;
mod eventfd;
mod eventfd_io;
pub mod family;
pub mod flags;
mod fsync;
mod held;
mod link;
mod locks;
mod made;
mod memfd;
mod memfd_map;
mod meta;
mod mknod;
pub mod models;
mod open;
mod owner;
mod path;
mod pread;
mod private;
mod read;
mod regular;
mod rename;
mod resolve;
mod root;
mod seek;
mod slot;
mod store;
mod store_name;
mod system;
mod timerfd;
mod timerfd_read;
mod timerfd_spec;
mod walk;
mod write;
mod xattrs;

pub use at::join;
pub use close::close;
pub use cstr::read_cstr;
pub use dev_io::{read as dev_read, write as dev_write};
pub use dirents::getdents64;
pub use dirops::{mkdirat, rmdir, unlinkat};
pub use epoll::{epoll_create, epoll_ctl};
pub use epoll_arm::rearm;
pub use epoll_wait::epoll_wait;
pub use eventfd::{bits as event_bits, eventfd2};
pub use eventfd_io::{read as event_read, write as event_write};
pub use fsync::{fsync, sync, syncfs};
pub use link::{linkat, symlinkat};
pub use memfd::{is_memfd, memfd_create};
pub use memfd_map::{mapped_at, set_mapped, staged};
pub use meta::{
    access, chmod, faccessat, fchmod, fchmodat, fstat, fstatfs, look, newfstatat, readlinkat,
    statfs, statx,
};
pub use mknod::mknodat;
pub use open::openat;
pub use owner::{fchown_ids, fchownat, utimensat, utimes};
pub use path::read_path;
pub use pread::{pread64, preadv, pwrite64, pwritev};
pub use private::{allow_shared_writes, clear as clear_private, prepare as prepare_private};
pub use read::read;
pub use rename::renameat2;
pub use resolve::{key, visible};
pub use seek::lseek;
pub use slot::{install, MAX_FDS};
pub use store::{read as store_read, write as store_write};
pub use timerfd::{timerfd_create, timerfd_gettime, timerfd_settime};
pub use timerfd_read::{bits as timer_bits, read as timerfd_read};
pub use walk::follow;
pub use write::write;
pub use {calls::*, held::*, locks::*, made::*, system::*, xattrs::*};
