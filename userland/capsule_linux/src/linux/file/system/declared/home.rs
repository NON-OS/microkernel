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
 * names, and the most every home in the store keeps together, as the Lead
 * ruled. A home lives in the store's memory for the boot, beside the
 * family's scratch, which keeps its own PRIVATE and PRIVATE_NAMES: a full
 * home is ENOSPC there and leaves /tmp writable, and a full /tmp leaves the
 * home. Every home together stays well inside the store's 320 MiB heap and
 * 2048 names, so no number of apps' homes ends the store for the rest.
 */
pub const HOME: u64 = 16 << 20;
pub const HOME_NAMES: u64 = 128;
pub const HOMES: u64 = 64 << 20;
// The store keeps 2048 names and a 320 MiB heap for every capsule: the
// build itself holds the ceilings inside it.
const _: () = assert!(HOMES <= 64 << 20 && HOME <= HOMES && HOME_NAMES <= 128);
