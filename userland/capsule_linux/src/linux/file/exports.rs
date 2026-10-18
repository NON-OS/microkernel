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

//! What the file layer gives the rest of the personality, named at `crate::linux::file` by mod.rs.

pub use super::at::join;
pub use super::close::close;
pub use super::dev_io::{read as dev_read, write as dev_write};
pub use super::dirents::getdents64;
pub use super::dirops::{mkdirat, rmdir, unlinkat};
pub use super::epoll::{epoll_create, epoll_ctl};
pub use super::epoll_arm::rearm;
pub use super::epoll_ready::bits as epoll_bits;
pub use super::epoll_wait::epoll_wait;
pub use super::eventfd::io::{read as event_read, write as event_write};
pub use super::eventfd::open::eventfd2;
pub use super::eventfd::ready::bits as event_bits;
pub use super::fsync::{fsync, sync, syncfs};
pub use super::link::{linkat, symlinkat};
pub use super::memfd::{is_memfd, memfd_create};
pub use super::memfd_map::{mapped_at, set_mapped, staged};
pub use super::meta::{
    access, chmod, faccessat, fchmod, fchmodat, fstat, fstatfs, look, newfstatat, readlinkat,
    statfs, statx,
};
pub use super::mknod::mknodat;
pub use super::open::openat;
pub use super::owner::{fchown_ids, fchownat, utimensat, utimes};
pub use super::path::name_of;
pub use super::pread::{pread64, preadv, pwrite64, pwritev};
pub use super::private::{allow_shared_writes, clear as clear_private, prepare as prepare_private};
pub use super::read::read;
pub use super::rename::renameat2;
pub use super::resolve::{key, visible};
pub use super::seek::lseek;
pub use super::slot::{install, MAX_FDS};
pub use super::store::{
    hash as store_hash, read as store_read, stat as store_stat, write as store_write, HashFault,
};
pub use super::store_found::found as store_found;
pub use super::store_name::unlink as store_unlink;
pub use super::timerfd::get::timerfd_gettime;
pub use super::timerfd::open::timerfd_create;
pub use super::timerfd::read::{bits as timer_bits, read as timerfd_read};
pub use super::timerfd::set::timerfd_settime;
pub use super::walk::follow;
pub use super::write::write;
pub use super::{calls::*, held::*, locks::*, made::*, system::*, xattrs::*};
