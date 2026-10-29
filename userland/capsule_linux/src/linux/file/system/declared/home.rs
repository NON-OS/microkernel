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

/*
 * The most a family's home (file/private/home.rs) may keep, in bytes and in
 * names, and the most every home in the store keeps together, as the developers
 * ruled. A home lives in the store's memory for the boot, beside the
 * family's scratch, which keeps its own PRIVATE and PRIVATE_NAMES: a full
 * home is ENOSPC there and leaves /tmp writable, and a full /tmp leaves the
 * home. A listing named in app/home_listings.rs keeps more, at most
 * HOME_MOST: Monero's wallet writes its cache whole beside the last, two
 * copies past 16 MiB each. Every home together stays inside the 160 MiB
 * the store gives its clients on the smallest machine, so no number of
 * apps' homes ends the store for the rest.
 */
pub const HOME: u64 = 16 << 20;
pub const HOME_MOST: u64 = 64 << 20;
pub const HOME_NAMES: u64 = 128;
pub const HOMES: u64 = 128 << 20;
// The store gives its clients' bytes no less than 160 MiB (capsule_vfs
// DATA_BYTES_MAX): the build itself holds the ceilings inside it.
const _: () = assert!(HOMES < 160 << 20 && HOME_MOST < HOMES && HOME < HOME_MOST);
const _: () = assert!(HOME_NAMES <= 128);
