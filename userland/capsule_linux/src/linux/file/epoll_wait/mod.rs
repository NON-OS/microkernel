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

//! `epoll_wait`: which of the watched descriptors are ready now.
//!
//! A level-triggered entry is reported for as long as its readiness holds.
//! An EPOLLET entry is reported when readiness rises: bits already seen at
//! the last look are left out until they fall, or until a call on the
//! descriptor answers EAGAIN (`epoll_arm`). An EPOLLONESHOT entry reports
//! once and then nothing until it is modified.

mod call;
mod collect;

pub use call::epoll_wait;
