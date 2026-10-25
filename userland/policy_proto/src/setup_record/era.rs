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
 * The desktop wallpaper and the wallpapers kept, in this catalog era.
 *
 * Only version 7 names them in this era. A version 6 record and every older
 * one named them by the first era's indices, which now point at other
 * pictures or at none, so their choice is not taken: they read as the default
 * wallpaper with every one kept, and the rest of the record is taken as it
 * was. Refusing them instead would drop their name, keyboard and network too.
 */

use super::layout::{ANSWERS_LEN, WALLPAPERS_AT};
use super::refused::Refused;
use crate::wallpaper_labels::WALLPAPER_DEFAULT;
use crate::wallpapers_kept::{kept, valid, ALL};

pub(super) fn wallpapers(version: u8, raw: &[u8]) -> Result<(u8, u64), Refused> {
    if version < 7 {
        return Ok((WALLPAPER_DEFAULT, ALL));
    }
    let set = raw.get(WALLPAPERS_AT..ANSWERS_LEN).ok_or(Refused::Length)?;
    let set = u64::from_le_bytes(set.try_into().map_err(|_| Refused::Length)?);
    // The desktop's wallpaper is one of those kept, and the set names only
    // wallpapers this catalog has.
    let wallpaper = raw[6];
    if !valid(set) || !kept(set, wallpaper) {
        return Err(Refused::Wallpapers);
    }
    Ok((wallpaper, set))
}
