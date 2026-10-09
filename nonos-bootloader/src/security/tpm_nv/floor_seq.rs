// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

//! The floor read and raised over any transport (TCG2, a scripted TPM, swtpm).
//!
//! The floor is how far the counter has risen since its base. A TPM starts a
//! new counter above the highest value any counter on it ever held, and
//! TPM2_Clear does not reset that, so a counter's own value says nothing of
//! this machine's releases: a TPM whose counters had counted (the loader's old
//! self-test counted every boot) read a floor in the thousands and refused
//! every release, through a TPM clear too. The base is the counter's value
//! when the floor began, written once to its own index and locked there.
//!
//! A counter the owner undefines comes back above the value it had, so the
//! floor it gives is above the old one, never 0 (REVIEW R20). A base the owner
//! undefines is set again from the counter, which starts the floor at 0: the
//! owner, who can clear the whole TPM, can reset it, and nobody else can.
//! An owner index with POLICY_DELETE is refused (TPM_RC_ATTRIBUTES).

use super::floor_cmd::{
    base_define, base_lock, base_read, base_write, floor_read, rb_define, rb_increment, rb_read,
    succeeded, FloorRead,
};

/// A command and its answer buffer to the bytes answered, `None` if unsent.
pub trait Submit: FnMut(&[u8], &mut [u8]) -> Option<usize> {}
impl<F: FnMut(&[u8], &mut [u8]) -> Option<usize>> Submit for F {}

/// The most increments one boot makes.
const MAX_RAISE: u32 = 4096;

fn read(submit: &mut impl Submit) -> FloorRead {
    read_at(submit, &rb_read())
}

fn read_at(submit: &mut impl Submit, cmd: &[u8]) -> FloorRead {
    let mut resp = [0u8; 64];
    match submit(cmd, &mut resp) {
        Some(n) => floor_read(&resp, n),
        None => FloorRead::Unreadable,
    }
}

fn increment(submit: &mut impl Submit) -> bool {
    let mut resp = [0u8; 64];
    submit(&rb_increment(), &mut resp).is_some_and(|n| succeeded(&resp, n))
}

/// The counter's value, defining it first; a define refused because the index
/// exists is the usual case. One never incremented is incremented once.
fn counter(submit: &mut impl Submit) -> Option<u64> {
    let mut resp = [0u8; 64];
    let _ = submit(&rb_define(), &mut resp);
    match read(submit) {
        FloorRead::Value(v) => Some(v),
        FloorRead::Uninitialized if increment(submit) => match read(submit) {
            FloorRead::Value(v) => Some(v),
            _ => None,
        },
        _ => None,
    }
}

/// The base, and whether it was set now: a base not yet written is set to the
/// counter's `value` and locked. One above the counter cannot come from this
/// loader and reads as none.
fn base(submit: &mut impl Submit, value: u64) -> Option<(u64, bool)> {
    let mut resp = [0u8; 64];
    let _ = submit(&base_define(), &mut resp);
    match read_at(submit, &base_read()) {
        FloorRead::Value(b) if b <= value => Some((b, false)),
        FloorRead::Uninitialized => {
            let mut resp = [0u8; 64];
            let written = submit(&base_write(value), &mut resp).is_some_and(|n| succeeded(&resp, n));
            if !written {
                return None;
            }
            /* A base left unlocked still holds the floor; locked, no write moves it. */
            let _ = submit(&base_lock(), &mut [0u8; 64]);
            Some((value, true))
        }
        _ => None,
    }
}

/// The floor, and whether its base was set on this read (a new TPM, or a base
/// the owner deleted).
pub fn floor_and_base_with(mut submit: impl Submit) -> Option<(u64, bool)> {
    let value = counter(&mut submit)?;
    let (b, set) = base(&mut submit, value)?;
    Some((value - b, set))
}

/// The floor.
pub fn floor_with(submit: impl Submit) -> Option<u64> {
    floor_and_base_with(submit).map(|(floor, _)| floor)
}

/// Raise the floor to `target`, one increment a step; true once it is at or
/// above it.
pub fn raise_with(mut submit: impl Submit, target: u64) -> bool {
    let Some(mut current) = floor_with(&mut submit) else {
        return false;
    };
    let mut steps = 0u32;
    while current < target {
        if steps >= MAX_RAISE || !increment(&mut submit) {
            return false;
        }
        current += 1;
        steps += 1;
    }
    true
}
