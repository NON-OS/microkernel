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

//! Which igb function to take: the first copper port, else the first of
//! any media. Bring-up refuses every media but copper, so a fiber or SerDes
//! port earlier in the device list must not shadow a copper one after it.
//! Pure, so igb_proofs holds it.

pub fn choose<T>(found: impl Iterator<Item = T>, copper: impl Fn(&T) -> bool) -> Option<T> {
    let mut fallback = None;
    for f in found {
        if copper(&f) {
            return Some(f);
        }
        fallback.get_or_insert(f);
    }
    fallback
}
