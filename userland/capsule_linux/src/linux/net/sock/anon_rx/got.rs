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

//! What one read from net.anon brought.

use alloc::vec::Vec;

/// What one read from net.anon brought.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Got {
    Bytes(Vec<u8>),
    /// Nothing yet; the stream is open.
    Nothing,
    /// The far end finished, with this END reason.
    End(u8),
    /// net.anon holds no such stream for this capsule.
    Gone,
    /// An answer this capsule cannot read.
    Garbled,
    /// No answer.
    Silent,
}
