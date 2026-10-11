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

use alloc::vec::Vec;

use nonos_marketplace_abi::{decode_index, encode_and_sign, DecodeError, MarketplaceIndex};

pub(crate) fn blob(index: MarketplaceIndex) -> Vec<u8> {
    encode_and_sign(index, |_| [7; 64]).expect("a well formed index encodes").0
}

pub(crate) fn decode(blob: &[u8]) -> Result<MarketplaceIndex, DecodeError> {
    decode_index(blob).map(|d| d.index)
}

/// Where `needle` starts in `hay`, for damaging one field of an encoded blob.
pub(crate) fn find(hay: &[u8], needle: &[u8]) -> usize {
    hay.windows(needle.len()).position(|w| w == needle).expect("the field is in the blob")
}
