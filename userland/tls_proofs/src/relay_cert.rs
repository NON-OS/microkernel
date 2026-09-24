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

//! The certificate 46.37.123.145:9001 served on 2026-09-20.

pub const RELAY_CERT: &[u8] = include_bytes!("relay_cert.der");

/// The span openssl reports for the public key inside it.
pub const SPKI_AT: usize = 146;
pub const SPKI_LEN: usize = 294;
