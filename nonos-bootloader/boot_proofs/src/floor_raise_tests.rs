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

//! Raising the floor after a verified kernel (REVIEW R20).

use crate::scripted_tpm::{Tpm, CC_INCREMENT};
use crate::security::tpm_nv::floor_seq::raise_with;

fn raise(t: &mut Tpm, target: u64) -> bool {
    raise_with(|c: &[u8], o: &mut [u8]| t.answer(c, o), target)
}

fn increments(t: &Tpm) -> usize {
    t.log.iter().filter(|&&c| c == CC_INCREMENT).count()
}

#[test]
fn raising_reaches_the_target_and_never_lowers() {
    let mut t = Tpm::holding(13, 10);
    assert!(raise(&mut t, 5));
    assert_eq!((t.value, increments(&t)), (Some(15), 2));
    let mut t = Tpm::holding(19, 10);
    assert!(raise(&mut t, 5), "a floor above the target holds it");
    assert_eq!((t.value, increments(&t)), (Some(19), 0));
    let mut t = Tpm::default();
    assert!(raise(&mut t, 1), "a new TPM");
    assert_eq!((t.value, t.base), (Some(2), Some(1)));
}

#[test]
fn a_failed_increment_or_an_unreadable_counter_is_not_raised() {
    let mut t = Tpm { fail_increment: true, ..Tpm::holding(3, 0) };
    assert!(!raise(&mut t, 4));
    let mut t = Tpm { read_rc: Some(0x18B), ..Default::default() };
    assert!(!raise(&mut t, 1));
}
