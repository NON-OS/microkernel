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


//! Onion services on the Anyone network: the client side.
//!
//! The fork keeps Tor's v3 onion services and renames the suffix and the
//! checksum prefix. Everything here is pure and held in anon_ntor_proofs
//! against the fork's own known answers; manager/onion drives it over the
//! network.

pub mod address;
pub mod blind;
pub mod cache;
pub mod cells;
pub mod cert;
pub mod client_auth;
pub mod desc;
pub mod fetch;
pub mod hs_ntor;
pub mod lookup;
pub mod names;
pub mod period;
pub mod pow;
pub mod ring;
