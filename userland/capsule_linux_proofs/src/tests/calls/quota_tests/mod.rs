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

//! What a family may keep in its private directories. They live in the
//! store every capsule shares, which holds 2048 names in all and keeps its
//! bytes in one 192 MiB heap, so the quota is what keeps one guest from
//! filling the store or running it out of memory. Held here: the ceilings
//! are ones the store can carry, a family is refused past either with
//! ENOSPC however it gets there, a family over the quota can still shrink,
//! and the copies it writes into, held in this capsule's memory until they
//! reach the store, stop at the same ceiling.

mod ceiling;
mod growth;
mod home;
mod home_key;
mod open;
