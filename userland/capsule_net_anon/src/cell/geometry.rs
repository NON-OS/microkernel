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

//! Cell and relay-header offsets, from or.h and tor-spec section 3.

pub const PAYLOAD_BYTES: usize = 509;

/// CircID plus command, at link protocol 4 or later. Link 3 and earlier used a
/// two byte CircID; this capsule negotiates 4 or 5 only.
pub const HEADER_BYTES: usize = 5;

pub const CELL_BYTES: usize = HEADER_BYTES + PAYLOAD_BYTES;

/// command, recognized, stream_id, digest, length.
pub const RELAY_HEADER_BYTES: usize = 1 + 2 + 2 + 4 + 2;

pub const RELAY_BODY_BYTES: usize = PAYLOAD_BYTES - RELAY_HEADER_BYTES;

pub const RELAY_DIGEST_AT: usize = 5;
pub const RELAY_DIGEST_BYTES: usize = 4;
pub const RELAY_LENGTH_AT: usize = 9;

/// Cap on a variable length cell body. A CERTS cell runs to a few kilobytes; a
/// peer claiming more than this sizes no allocation.
pub const VARIABLE_MAX: usize = 64 * 1024;
