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
 * Who the guest is beyond its uid: its three ids, its groups, its nice
 * value, and its execution domain.
 *
 * Every id the personality reports is root's, and a guest holds no
 * capability (capget is refused), so Linux's rule for a process without
 * CAP_SETUID applies: it may set an id only to one it already has, which
 * here is 0. A nice value is kept and reported, not acted on: the
 * family's threads are scheduled by NONOS, which does not read it.
 */

mod creds;
mod nice;
mod who;

pub use creds::{getgroups, getres, personality, setgroups, setres};
pub use nice::{getpriority, nice_of, setpriority};
