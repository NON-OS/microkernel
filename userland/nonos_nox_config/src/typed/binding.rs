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

//! The payee's signature over a listing's Ed25519 publisher key. It
//! is what lets the bond contract slash for something that key signed.

use super::TypedData;
use crate::domain::BOND;
use crate::hex::hex32;

pub const PUBLISHER_BINDING: TypedData = TypedData {
    type_string: "PublisherBinding(bytes32 publisherKey,bytes32 listingId,bytes32 bondId)",
    typehash: hex32("f3a44af7b15b9b54ed9ce73eb2a58f9b0154713e3265acc20e98701170b1c88f"),
    domain: &BOND,
    fields: 3,
};
