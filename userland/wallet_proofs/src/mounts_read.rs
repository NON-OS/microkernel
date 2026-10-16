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

//! The read files: last read, probe reading, refresh due time, route waits, records, layout.

#[path = "../../capsule_wallet_nonos/src/wallet/net/last_read.rs"]
pub mod last_read;
#[path = "../../capsule_wallet_nonos/src/wallet/etna/layout.rs"]
pub mod layout;
#[path = "../../capsule_wallet_nonos/src/wallet/event/probe/reading.rs"]
pub mod reading;
#[path = "../../capsule_wallet_nonos/src/wallet/tls13/records_whole.rs"]
pub mod records_whole;
#[path = "../../capsule_wallet_nonos/src/wallet/app_refresh_due.rs"]
pub mod refresh_due;
/// The route link's waits, which a refresh's steps keep to.
#[path = "../../nonos_route_link/src/bounds.rs"]
pub mod route_bounds;
