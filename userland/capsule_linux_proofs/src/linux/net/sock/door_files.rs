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
 * The family's server path as SERVE_ONE's door goes through it, mounted in
 * sock/mod.rs where its own `super::` paths look: the door end autobound
 * and joined to a listener, accept, the room a write is held to, a read,
 * and the bits poll reports.
 */
#[path = "../../../../../capsule_linux/src/linux/net/sock/accepted.rs"]
mod accepted;
#[path = "../../../../../capsule_linux/src/linux/net/sock/bind.rs"]
mod bind;
#[path = "../../../../../capsule_linux/src/linux/net/sock/door.rs"]
mod door;
#[path = "../../../../../capsule_linux/src/linux/net/sock/link.rs"]
mod link;
#[path = "../../../../../capsule_linux/src/linux/net/sock/port.rs"]
mod port;
#[path = "../../../../../capsule_linux/src/linux/net/sock/put.rs"]
mod put;
#[path = "../../../../../capsule_linux/src/linux/net/sock/ready.rs"]
mod ready;
#[path = "../../../../../capsule_linux/src/linux/net/sock/recv.rs"]
mod recv;
#[path = "../../../../../capsule_linux/src/linux/net/sock/send.rs"]
mod send;
#[path = "../../../../../capsule_linux/src/linux/net/sock/syn.rs"]
mod syn;
