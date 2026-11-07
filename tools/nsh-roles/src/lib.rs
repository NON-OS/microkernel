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

//! The fleet operator's side of nsh/1 roles: the role table, the
//! revocation list, grants and their approvals. Each is laid out field by
//! field, signed with both halves of a hybrid key, and opened or checked by
//! `nsh_roles` exactly as a host does before it is handed back, so a file
//! this tool writes is one a host accepts.

pub mod approve;
pub mod error;
pub mod grant;
pub mod keys;
mod put;
pub mod revoke;
pub mod roles_file;
pub mod shipped;
pub mod show;
pub mod table;

pub use error::Error;
