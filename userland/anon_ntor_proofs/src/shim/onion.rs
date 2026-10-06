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


//! The capsule's onion service modules, file by file as each one lands.

#[path = "../../../capsule_net_anon/src/onion/address.rs"]
pub mod address;
#[path = "../../../capsule_net_anon/src/onion/blind.rs"]
pub mod blind;
#[path = "../../../capsule_net_anon/src/onion/period.rs"]
pub mod period;
#[path = "../../../capsule_net_anon/src/onion/ring.rs"]
pub mod ring;
#[path = "../../../capsule_net_anon/src/onion/lookup.rs"]
pub mod lookup;
#[path = "../../../capsule_net_anon/src/onion/cert.rs"]
pub mod cert;
#[path = "../../../capsule_net_anon/src/onion/desc/mod.rs"]
pub mod desc;
#[path = "../../../capsule_net_anon/src/onion/fetch.rs"]
pub mod fetch;
#[path = "../../../capsule_net_anon/src/onion/hs_ntor.rs"]
pub mod hs_ntor;
#[path = "../../../capsule_net_anon/src/onion/cells.rs"]
pub mod cells;
#[path = "../../../capsule_net_anon/src/onion/cache.rs"]
pub mod cache;
#[path = "../../../capsule_net_anon/src/onion/client_auth.rs"]
pub mod client_auth;
#[path = "../../../capsule_net_anon/src/onion/pow/mod.rs"]
pub mod pow;
