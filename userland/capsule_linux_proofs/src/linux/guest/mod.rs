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

//! The pure part of the capsule's guest: page arithmetic and the layout of a
//! guest's address space, re-exported under the names the capsule's
//! `guest` module gives them.

#[path = "../../../../capsule_linux/src/linux/guest/fd_kind.rs"]
pub mod fd_kind;
#[path = "../../../../capsule_linux/src/linux/guest/futex_pick.rs"]
pub mod futex_pick;
#[path = "../../../../capsule_linux/src/linux/guest/layout.rs"]
pub mod layout;
#[path = "../../../../capsule_linux/src/linux/guest/links_room.rs"]
pub mod links_room;
#[path = "../../../../capsule_linux/src/linux/guest/mem.rs"]
pub mod mem;
#[path = "../../../../capsule_linux/src/linux/guest/memory.rs"]
pub mod memory;
#[path = "../../../../capsule_linux/src/linux/guest/sigalt.rs"]
pub mod sigalt;
#[path = "../../../../capsule_linux/src/linux/guest/sigrestart.rs"]
pub mod sigrestart;
#[path = "../../../../capsule_linux/src/linux/guest/sigstate.rs"]
pub mod sigstate;
#[path = "../../../../capsule_linux/src/linux/guest/slots.rs"]
pub mod slots;
#[path = "../../../../capsule_linux/src/linux/guest/timer.rs"]
pub mod timer;
#[path = "../../../../capsule_linux/src/linux/guest/watch.rs"]
pub mod watch;

pub use fd_kind::Kind;
pub use layout::{MMAP_BASE, MMAP_LIMIT, STACK_SIZE, USER_MAX};
pub use mem::{maps_full, page_down, page_len, page_up, span_within, MAX_MAPS, MAX_SPAN, PAGE};
pub use watch::{EPOLLET, EPOLLONESHOT};
