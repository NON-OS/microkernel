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

//! Pseudo-terminals, as a terminal emulator such as foot opens them: a
//! master from /dev/ptmx, its number from TIOCGPTN, unlocked with
//! TIOCSPTLCK, and its slave from /dev/pts/N, which a shell started with
//! setsid and TIOCSCTTY takes as its controlling terminal. What the master
//! writes passes the slave's line discipline (canonical mode, echo, ^C as
//! SIGINT for the foreground group); what the slave writes passes output
//! processing. The pairs are the family's, kept in one table like the
//! console, and a reader or writer that would wait parks as on a pipe.

mod cell;
mod disc;
mod edit;
mod fd;
pub mod flags;
mod input;
mod io;
mod ioctl;
mod ioctl_ctty;
mod ioctl_job;
mod ioctl_set;
mod lookup;
mod make;
mod name;
mod note;
mod open;
mod out;
mod pair;
mod pair_io;
mod side;
mod side_poll;
pub mod table;
mod table_io;

pub use fd::{bits, names, note, signals};
pub use io::{read, write};
pub use ioctl::ioctl;
pub use name::pts_number;
pub use open::open;
