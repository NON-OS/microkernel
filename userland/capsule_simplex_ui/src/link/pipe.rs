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

//! What carries the core's bytes: one SERVE_ONE connection in the window,
//! a scripted one in the proofs.

use alloc::vec::Vec;

/// One connection to the core's port, as SERVE_ONE serves it.
pub trait Pipe {
    /// Bytes that came since the last read; None once the connection is gone.
    fn read(&mut self) -> Option<Vec<u8>>;
    /// False when the bytes could not be sent.
    fn write(&mut self, bytes: &[u8]) -> bool;
}
