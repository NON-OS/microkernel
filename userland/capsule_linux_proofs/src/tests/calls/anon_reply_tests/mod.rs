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

//! What the capsule says to net.anon's handle front and how it reads the
//! answers. The values are held to net.anon's own files, the header to the
//! lines that write and read it there, and every reply, however short,
//! long, garbled or unknown, ends in a clean errno and never a panic.

mod header;
mod host;
mod hostile;
mod moves;
mod open;
mod reply;
mod server;
mod values;
