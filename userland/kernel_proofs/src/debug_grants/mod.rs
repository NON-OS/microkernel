// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

//! The real-hardware drivers whose own lines a development image's console
//! carries: each spawn folds the manifest's optional Debug in through
//! serial_debug_cap(), which is 0 unless the kernel is built with
//! capsule-serial-debug, and each manifest declares it optional (0x100).

#[cfg(test)]
mod drivers;
#[cfg(test)]
mod tests;
