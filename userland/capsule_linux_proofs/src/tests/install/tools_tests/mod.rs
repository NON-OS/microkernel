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

//! An in-tree tool the image does not carry: the market lists it as
//! `linux.nonos-<tool>`, the kernel hands the personality `nonos-<tool>` in
//! Alpine's family, where the stored tools live, and the personality fetches
//! the content its pin names from the NONOS package mirror. Held end to end:
//! the kernel's own mapping, the personality's split and its reading of the
//! name, and the path a mirror serves the content at.

mod name;
mod seal;
