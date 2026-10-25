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

//! The payee's word on one release's bytes, carried in the release
//! as release_attest_sig. Two of these by one payee for the same listing and
//! release with any field different are equivocation, and anyone may slash.
//! An escrow release sets packageHash to zero; any other sets chunkRoot to
//! zero and contentLen to 0.

use super::TypedData;
use crate::domain::BOND;
use crate::hex::hex32;

pub const RELEASE_ATTEST: TypedData = TypedData {
    type_string: "ReleaseAttest(bytes32 listingId,bytes32 releaseId,bytes32 bondId,\
bytes32 chunkRoot,uint64 contentLen,bytes32 packageHash)",
    typehash: hex32("2c3027b70e6539f80e45782d694163977de6df1bafcba6fa319d7a6c6283faf0"),
    domain: &BOND,
    fields: 6,
};
