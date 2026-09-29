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
 * /dev: the names a Linux program finds there, with Linux's numbers, and
 * the links into /proc/self/fd. The devices themselves, null, zero, full,
 * random and urandom, are descriptors file/dev.rs answers.
 *
 * There is no terminal: a guest's console is a stream, as a pipe is, so
 * /dev/tty answers ENXIO, which is what Linux answers a process with no
 * controlling terminal. /dev/shm is the family's own private directory in
 * the store, and is not made here.
 */

mod number;
mod tree;

pub use number::{at, rdev};
pub use tree::{node, Dev};
