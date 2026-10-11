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

//! What a guest's state gives the rest of the personality, named at `crate::linux::guest` by mod.rs.

pub use super::blocked::Blocked;
pub use super::event::Event;
pub use super::fd::Fd;
pub use super::fd_kind::Kind;
pub use super::handle::Guest;
pub use super::layout::{
    BRK_BASE, BRK_LIMIT, EXEC_BASE, INTERP_BASE, MMAP_BASE, MMAP_LIMIT, STACK_SIZE, STACK_TOP,
    USER_MAX,
};
pub use super::links::Links;
pub use super::mem::{maps_full, page_down, page_len, page_up, span_within, MAX_SPAN, PAGE};
pub use super::region::{peer_prot, Region};
pub use super::timer::Timer;
pub use super::watch::{Watch, EPOLLET, EPOLLONESHOT};
