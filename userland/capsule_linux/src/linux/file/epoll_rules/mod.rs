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

//! epoll's rules, apart from any descriptor: what one look at an entry
//! reports, which event masks EPOLL_CTL refuses, and how far one epoll may
//! watch another. Pure, so the host proofs hold each of them.

mod ctl;
mod look;
mod nest;

pub use ctl::exclusive_ok;
pub use look::{live, seen, EPOLL_CTL_ADD, EPOLL_CTL_MOD};
pub use nest::{loops, MAX_NESTS};
