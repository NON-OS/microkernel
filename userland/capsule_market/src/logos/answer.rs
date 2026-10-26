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

//! Whether a listing's logo is served: only bytes whose BLAKE3 is the
//! logo_hash its signed entry names, and only as a raster the wire carries.

use nonos_market_proto::logo::logo_side;

/// What OP_GET_LOGO answers for one listing.
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Served<'a> {
    /// The entry names no logo, or this image carries none for it (an
    /// operator's index newer than the image): the store draws its own.
    None,
    Logo(&'a [u8]),
    /// The image's bytes are not the ones the index signs.
    Refused,
}

/// The answer for an entry whose signed logo_hash is `signed`, given the
/// bytes this image carries for it, if any.
pub(crate) fn served<'a>(signed: &[u8; 32], carried: Option<&'a [u8]>) -> Served<'a> {
    let Some(bytes) = carried.filter(|_| *signed != [0; 32]) else {
        return Served::None;
    };
    if blake3::hash(bytes).as_bytes() != signed || logo_side(bytes.len()).is_none() {
        return Served::Refused;
    }
    Served::Logo(bytes)
}
