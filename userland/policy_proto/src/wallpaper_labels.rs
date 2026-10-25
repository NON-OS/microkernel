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
 * The wallpapers' names, in catalog order (nonos-data/wallpapers/catalog.txt):
 * a wallpaper's index is its place in this list, the setting's value, and its
 * bit in the kept set. wallpaper_catalog_proofs checks the list against the
 * catalog's pins, so a picture added to one and not the other fails there.
 */
pub const WALLPAPER_LABELS: &[&[u8]] =
    &[b"Canyon neon", b"Canyon real", b"Canyon real soft", b"Neon sign"];

/*
 * The catalog's era. Era 1 was the 63 pictures of 0.9.2; this is era 2. A
 * record kept in an earlier era names its wallpaper by that era's index, and
 * the readers (setup_record, settings_record) take such a choice as the
 * default instead of as the picture now at that index.
 */
pub const CATALOG_ERA: u8 = 2;

/*
 * The desktop's wallpaper until someone picks another: the policy store's
 * default, setup's first choice, and the picture the wallpaper service links
 * in for its first frame (wallpaper_catalog_proofs pins all three to it).
 */
pub const WALLPAPER_DEFAULT: u8 = 0;
