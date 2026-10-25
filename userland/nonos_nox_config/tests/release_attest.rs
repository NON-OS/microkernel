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

/// The keyring pins the same hash for table entry 3 (plan/31), so a change
/// here must change there in the same pair of units.
#[test]
fn release_attest_is_the_v1_1_type() {
    let t = nonos_nox_config::RELEASE_ATTEST;
    assert_eq!(
        t.type_string,
        "ReleaseAttest(bytes32 listingId,bytes32 releaseId,bytes32 bondId,bytes32 chunkRoot,uint64 contentLen,bytes32 packageHash)"
    );
    assert_eq!(
        nonos_nox_config::hex32("2c3027b70e6539f80e45782d694163977de6df1bafcba6fa319d7a6c6283faf0"),
        t.typehash
    );
}
