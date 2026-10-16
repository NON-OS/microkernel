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

#[path = "../../../../src/process/foreign/frame.rs"]
mod frame;
#[path = "../../../../src/process/foreign/registry.rs"]
mod registry;
pub mod seal;
mod trap_reply;
#[path = "../../../../src/process/foreign/trap_table.rs"]
pub mod trap_table;
#[cfg(test)]
mod tests;
