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
 * HTTPS to a mirror: one GET for a file from a byte on, the certificate
 * chain checked against the built-in roots for the host before the request
 * is written, redirects followed to other HTTPS hosts, and the body read as
 * it arrives, never held whole.
 */

mod body;
mod fault;
mod head;
mod head_read;
mod open;
mod request;

pub use body::Body;
pub use fault::Fault;
pub use open::open;
