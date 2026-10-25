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

//! The stack canary (security/hardening/memory_sanitization/canary.rs) is
//! drawn only from the seeded RNG, and kept as it was when none answers.
//! The erase passes' overwrite seeds (erase.rs) take the weak word on
//! purpose: a wipe never waits on, or is refused by, the seeded RNG.

use crate::security::hardening::memory_sanitization::canary::{
    get_stack_canary, init_stack_canary,
};
use crate::security::hardening::memory_sanitization::erase::{dod_5220_erase, gutmann_erase};
use crate::stand_in::rng;

#[test]
fn the_canary_refuses_with_no_randomness_and_stays_as_it_was() {
    let before = get_stack_canary();
    rng::refuse();
    assert_eq!(init_stack_canary(), Err("no randomness"));
    assert_eq!(get_stack_canary(), before);
    rng::fail_from(None);
    init_stack_canary().expect("canary");
    let drawn: [u8; 8] = rng::given()[0][..].try_into().expect("8 bytes");
    assert_eq!(get_stack_canary(), u64::from_le_bytes(drawn));
    assert_ne!(get_stack_canary(), before);
    assert_eq!(rng::weak(), 0);
}

#[test]
fn the_erase_passes_wipe_with_no_seeded_draw() {
    for erase in [dod_5220_erase, gutmann_erase] {
        let mut buf = [0x33u8; 64];
        rng::refuse();
        erase(buf.as_mut_ptr(), buf.len());
        assert_eq!(buf, [0; 64]);
        assert_eq!((rng::draws(), rng::weak()), (0, 1));
    }
}
