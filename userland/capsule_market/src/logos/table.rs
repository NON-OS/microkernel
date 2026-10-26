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

//! The logo rasters a maintainer made once and committed under
//! nonos-data/market/logos/ (tools/nonos_market_catalogue/logos.py),
//! embedded as they are. The test beside the catalogue tool holds this
//! list to the recipes that name a logo.

macro_rules! logo {
    ($id:literal) => {
        ($id, include_bytes!(concat!("../../../../nonos-data/market/logos/", $id, ".rgba")))
    };
}

static LOGOS: [(&str, &[u8]); 11] = [
    logo!("linux.bat"),
    logo!("linux.flac"),
    logo!("linux.fzf"),
    logo!("linux.git"),
    logo!("linux.helix"),
    logo!("linux.kakoune"),
    logo!("linux.lame"),
    logo!("linux.micro"),
    logo!("linux.nnn"),
    logo!("linux.profanity"),
    logo!("linux.vim"),
];

/// The bytes this image carries for `listing_id`, if any.
pub(crate) fn embedded(listing_id: &str) -> Option<&'static [u8]> {
    LOGOS.iter().find(|(id, _)| *id == listing_id).map(|(_, bytes)| *bytes)
}
