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

mod commerce_bytes;
mod commerce_from_bytes;
mod decode_commerce;
mod decode_entry;
mod decode_index;
mod decode_price;
mod decode_release;
mod decode_release_ext;
mod decode_token;
mod decode_validation;
#[cfg(feature = "canonical-encode")]
mod encode_check;
#[cfg(feature = "canonical-encode")]
mod encode_entry;
#[cfg(feature = "canonical-encode")]
mod encode_error;
#[cfg(feature = "canonical-encode")]
mod encode_index;
#[cfg(feature = "canonical-encode")]
mod encode_price;
#[cfg(feature = "canonical-encode")]
mod encode_release;
#[cfg(feature = "canonical-encode")]
mod encode_token;
#[cfg(feature = "canonical-encode")]
mod encode_validation;
mod entry_document;
mod error;
mod reader;
mod release_document;
mod release_signing;
mod schema;
mod strings;
mod write_release_ext;
mod writer;

pub use commerce_bytes::{commerce_bytes, commerce_hash_input, COMMERCE_DOMAIN};
pub use commerce_from_bytes::commerce_from_bytes;
pub use decode_index::{decode_index, DecodedIndex};
#[cfg(feature = "canonical-encode")]
pub use encode_error::EncodeError;
#[cfg(feature = "canonical-encode")]
pub use encode_index::{encode_and_sign, encode_index, EncodedIndex};
pub use entry_document::decode_entry_document;
#[cfg(feature = "canonical-encode")]
pub use entry_document::encode_entry_document;
pub use error::DecodeError;
pub use release_document::decode_release_document;
#[cfg(feature = "canonical-encode")]
pub use release_document::encode_release_document;
pub use release_signing::release_signing_bytes;
pub use schema::{CURRENT_SCHEMA, SCHEMA_V2, SCHEMA_V3};
