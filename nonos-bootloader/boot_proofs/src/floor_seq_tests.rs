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

//! The loader's floor read against a TPM that keeps its counter and the
//! counter's base as the specification does (REVIEW R20).

use crate::scripted_tpm::{Tpm, CC_DEFINE, CC_INCREMENT, CC_READ, CC_WRITE, CC_WRITELOCK};
use crate::security::tpm_nv::floor_seq::{floor_and_base_with, floor_with, raise_with};

fn floor(t: &mut Tpm) -> Option<u64> {
    floor_with(|c: &[u8], o: &mut [u8]| t.answer(c, o))
}

fn floor_and_set(t: &mut Tpm) -> Option<(u64, bool)> {
    floor_and_base_with(|c: &[u8], o: &mut [u8]| t.answer(c, o))
}

fn raise(t: &mut Tpm, target: u64) -> bool {
    raise_with(|c: &[u8], o: &mut [u8]| t.answer(c, o), target)
}

#[test]
fn a_new_tpm_starts_the_floor_at_zero_and_locks_its_base() {
    let mut t = Tpm::default();
    assert_eq!(floor_and_set(&mut t), Some((0, true)));
    assert_eq!((t.value, t.base, t.base_locked), (Some(1), Some(1), true));
    assert_eq!(
        t.log,
        [CC_DEFINE, CC_READ, CC_INCREMENT, CC_READ, CC_DEFINE, CC_READ, CC_WRITE, CC_WRITELOCK]
    );
}

/// The machine that refused every release: its TPM's counters had reached
/// 6880 under the old self-test, so a new rollback counter started at 6881,
/// a TPM clear included. The floor is the rise above that, 0.
#[test]
fn a_tpm_whose_counters_counted_still_starts_at_zero() {
    let mut t = Tpm { max: 6880, ..Default::default() };
    assert_eq!(floor(&mut t), Some(0));
    assert!(raise(&mut t, 1), "release 1 boots and raises the floor to it");
    assert_eq!((floor(&mut t), t.value), (Some(1), Some(6882)));
    let mut cleared = Tpm::holding(6881, 6881);
    cleared.clear();
    assert_eq!(floor(&mut cleared), Some(0), "after TPM2_Clear too");
}

/// A machine raised by a loader that kept no base: the counter it left becomes
/// the base, so the release that boots there sets the floor again.
#[test]
fn a_counter_without_a_base_becomes_the_base() {
    let mut t = Tpm { defined: true, value: Some(6881), max: 6881, ..Default::default() };
    assert_eq!(floor_and_set(&mut t), Some((0, true)));
    assert_eq!((t.base, t.base_locked), (Some(6881), true));
    assert_eq!(floor_and_set(&mut t), Some((0, false)), "then held, not set again");
}

#[test]
fn a_held_floor_is_read_and_nothing_written() {
    let mut t = Tpm::holding(9, 4);
    assert_eq!(floor_and_set(&mut t), Some((5, false)));
    assert_eq!(t.log, [CC_DEFINE, CC_READ, CC_DEFINE, CC_READ]);
}

#[test]
fn a_counter_the_owner_undefined_reads_above_its_old_floor() {
    for (counter, base) in [(1, 1), (5, 1), (40, 7), (u64::from(u32::MAX), 3)] {
        let mut t = Tpm::holding(counter, base);
        let old = counter - base;
        t.undefine();
        assert_eq!(floor(&mut t), Some(old + 1), "was {old}");
        assert_eq!(floor(&mut t), Some(old + 1), "then held");
    }
}

#[test]
fn a_base_the_owner_undefined_is_set_again_and_said() {
    let mut t = Tpm::holding(9, 4);
    t.undefine_base();
    assert_eq!(floor_and_set(&mut t), Some((0, true)));
    assert_eq!(t.base, Some(9));
}

#[test]
fn a_locked_base_takes_no_write() {
    let mut t = Tpm::holding(9, 4);
    let mut out = [0u8; 64];
    let n = t.answer(&crate::security::tpm_nv::floor_cmd::base_write(9), &mut out).unwrap();
    assert_eq!(u32::from_be_bytes(out[6..10].try_into().unwrap()), 0x148, "{n} bytes: NV locked");
    assert_eq!(floor(&mut t), Some(5));
}

#[test]
fn a_base_above_its_counter_or_unwritable_reads_none() {
    let mut t = Tpm::holding(4, 9);
    assert_eq!(floor(&mut t), None);
    let mut t = Tpm { fail_base_write: true, ..Default::default() };
    assert_eq!(floor(&mut t), None, "a base that could not be written holds nothing");
}

#[test]
fn an_uninitialized_counter_that_cannot_be_raised_reads_none() {
    let mut t = Tpm { fail_increment: true, ..Default::default() };
    assert_eq!(floor(&mut t), None);
    let mut t = Tpm { read_rc: Some(0x14A), ..Default::default() };
    assert_eq!(floor(&mut t), None, "still uninitialized after the increment");
    assert_eq!(t.log, [CC_DEFINE, CC_READ, CC_INCREMENT, CC_READ]);
}

#[test]
fn every_other_read_answer_reads_none_and_increments_nothing() {
    // handle, failure, retry, NV locked, authorization failure, size
    for rc in [0x18B, 0x101, 0x922, 0x148, 0x98E, 0x095] {
        let mut t = Tpm { read_rc: Some(rc), ..Tpm::holding(9, 1) };
        assert_eq!(floor(&mut t), None, "rc {rc:#x}");
        assert_eq!(t.log, [CC_DEFINE, CC_READ], "rc {rc:#x}");
    }
    assert_eq!(floor_with(|_: &[u8], _: &mut [u8]| None), None, "nothing reached a TPM");
}
