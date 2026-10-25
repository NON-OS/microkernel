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
//
// Host-side harness for the block's context line and the identity behind it.
//
// `wire.rs` is written against the real wire crate rather than a mirror of it,
// because a mirror would pass while the ABI drifted underneath it, so that
// crate is a dependency of this one.
//
// The IPC and the cache need a kernel, so neither is pulled in. What is under
// test is everything that decides what gets drawn: the wire framing, the
// sanitizing of a reply, the fallback choice, and the formatting.

#[path = "context_host/context_line.rs"]
mod context_line;
#[path = "context_host/home.rs"]
mod home;
#[path = "context_host/hostname.rs"]
mod hostname;
#[path = "context_host/mounts.rs"]
mod mounts;
#[path = "context_host/shorten.rs"]
mod shorten;
#[path = "context_host/support.rs"]
mod support;
#[path = "context_host/wire.rs"]
mod wire;

use mounts::term;
