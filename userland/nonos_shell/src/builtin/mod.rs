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

//! The builtins that need nothing of the terminal: each writes lines to a
//! `ShellOut` and asks the machine, if at all, through a `Host`. The GUI
//! terminal and a headless nsh session run these same files.

pub mod attest;
pub mod b64;
pub mod clock;
pub mod diff;
pub mod dns;
pub mod echo;
pub mod ed25519;
pub mod hash;
pub mod hex;
pub mod mem;
pub mod path;
pub mod ping;
pub mod printf;
pub mod ps;
pub mod rand;
pub mod seq;
pub mod service;
pub mod test;
