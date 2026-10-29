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

//! A package another app's run holds open is already in the store. The
//! install that needs it keeps that copy and goes on: the store will not
//! write over a file in use, and a whole install refused for a library a
//! running app shares (zlib, musl) reads as a fault the person cannot see.
//! Only for a package the install depends on and the store already has;
//! the program asked for is never kept in place of itself. Pure.

use alloc::string::String;

/// Whether `pkg`, refused because it is in use, is kept as it is.
pub fn keeps(in_use: bool, pkg: &str, asked: &str, installed: &[String]) -> bool {
    in_use && pkg != asked && installed.iter().any(|n| n == pkg)
}
