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

//! Building replies: a status alone, or a status, a prefix and data the
//! handler put after it. Each returns the reply's length.

use crate::wire::{encode_header, Request, HDR_LEN, STATUS_LEN};

/// Where a reply's data starts.
pub const DATA_AT: usize = HDR_LEN + STATUS_LEN;

pub fn status(tx: &mut [u8], req: &Request, status: i32) -> usize {
    encode_header(tx, req, status, 0)
}

/// A success reply whose `len` data bytes the handler already wrote at
/// DATA_AT.
pub fn data(tx: &mut [u8], req: &Request, len: usize) -> usize {
    encode_header(tx, req, 0, len) + len
}

/// A success reply carrying `bytes`.
pub fn bytes(tx: &mut [u8], req: &Request, bytes: &[u8]) -> usize {
    tx[DATA_AT..DATA_AT + bytes.len()].copy_from_slice(bytes);
    data(tx, req, bytes.len())
}
