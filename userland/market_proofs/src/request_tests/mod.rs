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

//! The market's request header decode and the length-prefixed readers its
//! OP_GET_APP, OP_GET_RELEASE and OP_INSTALL_READY bodies go through, with any
//! bytes. The header decode takes any frame without a panic and refuses what
//! is short or carries the wrong magic or version (the loop then answers with
//! a zero header and E_INVAL); the readers never read past the body, whatever
//! length a prefix claims.

mod frames;
mod header;
mod readers;
