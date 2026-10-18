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

//! K16: a capsule under or at the image ceiling spawns; one over it is
//! refused, and the refusal carries exactly the bits over.

use super::rule::gate;

/// IPC (3), Memory (4) and InputConsumer (36): input_router's grant.
const ROUTER: u64 = 1 << 3 | 1 << 4 | 1 << 36;

#[test]
fn under_and_at_the_ceiling_are_admitted() {
    assert_eq!(gate(1 << 3, ROUTER), Ok(()), "under");
    assert_eq!(gate(ROUTER, ROUTER), Ok(()), "at");
    assert_eq!(gate(0, 0), Ok(()), "nothing asked of nothing");
}

#[test]
fn over_the_ceiling_is_refused_with_the_bits_over() {
    let without_input = ROUTER & !(1 << 36);
    assert_eq!(gate(ROUTER, without_input), Err(1 << 36), "a ceiling without bit 36");
    assert_eq!(gate(u64::MAX, ROUTER), Err(!ROUTER));
    for bit in 0..64 {
        let asked = ROUTER | 1 << bit;
        let want = if ROUTER & 1 << bit != 0 { Ok(()) } else { Err(1 << bit) };
        assert_eq!(gate(asked, ROUTER), want, "bit {bit}");
    }
}
