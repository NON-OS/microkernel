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

//! The payee's word on one release's bytes. Two of these for the
//! same release with different roots are equivocation, and anyone may slash.

use super::TypedData;
use crate::domain::BOND;
use crate::hex::hex32;

pub const RELEASE_ATTEST: TypedData = TypedData {
    type_string: "ReleaseAttest(bytes32 listingId,bytes32 releaseId,bytes32 chunkRoot,\
bytes32 packageHash)",
    typehash: hex32("965a3a1c2dbb18f31f6c4c74711e07019f038524876eb7077b2ff59ff972ec4a"),
    domain: &BOND,
    fields: 4,
};
