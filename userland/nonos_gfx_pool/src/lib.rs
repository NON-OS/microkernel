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

//! A display driver's primaries as anonymous pages of its own (never device
//! memory), registered as ARGB8888 surfaces and shared so the compositor can
//! attach them: `nonos_gfx_serve`'s Platform over the kernel's calls.

#![no_std]

mod anon;
mod pool;
mod surface;

pub use pool::Pool;
