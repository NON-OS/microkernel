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

use nonos_policy_proto::{wallpapers_kept, Field};

use super::state::STORE;

/// Refuses a set the field cannot hold: no wallpaper kept, or one past the
/// collection.
pub fn set(field: Field, value: u64) -> bool {
    let mut s = STORE.lock();
    match field {
        Field::WallpapersKept if wallpapers_kept::valid(value) => s.wallpapers_kept = value,
        _ => return false,
    }
    super::state::changed();
    true
}
