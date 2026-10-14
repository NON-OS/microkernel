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

//! What the keyboard side has seen, as counts only: no byte is kept once it
//! is posted (plan/50 C8). OP_GET_STATE reports these.

#[derive(Clone, Copy, Default)]
pub struct Counters {
    /// Bytes read from the keyboard side of the controller.
    pub seen: u64,
    /// Key events the kernel's input ring refused.
    pub refused: u64,
    pub parity_errors: u64,
    pub timeout_errors: u64,
}
