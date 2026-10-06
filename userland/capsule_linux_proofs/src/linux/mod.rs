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

//! The capsule's pure call rules, mounted at the crate paths they name in
//! the capsule (`crate::linux::abi::errno`, `crate::linux::guest`), so each
//! compiles here exactly as it ships and finds what it uses where it looks.

pub mod abi;
pub mod call;
pub mod file;
pub mod guest;
pub mod net;
pub mod unix;
