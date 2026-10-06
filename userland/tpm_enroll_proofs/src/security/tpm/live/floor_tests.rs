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

//! The loader's rollback floor on a real TPM 2.0, and the owner undefining its
//! counter between two boots (REVIEW R20).

use super::swtpm::Swtpm;
use super::tools::tool;
use crate::loader_floor::floor_cmd::rb_define;
use crate::loader_floor::floor_seq::{floor_and_base_with, floor_with, raise_with};
use crate::security::tpm::transact;

fn submit(cmd: &[u8], out: &mut [u8]) -> Option<usize> {
    // SAFETY: the test's own swtpm, over TCP.
    unsafe { transact(cmd, out) }.ok()
}

#[test]
fn the_floor_survives_the_owner_undefining_its_counter() {
    let Some(t) = Swtpm::start("the_floor_survives_the_owner_undefining_its_counter") else {
        return;
    };
    assert_eq!(floor_with(submit), Some(0), "a new TPM");
    assert!(raise_with(submit, 5), "a kernel at index 5 verified");
    assert_eq!(floor_with(submit), Some(5));
    tool(Some(&t), "tpm2_nvundefine", &["0x01000020", "-C", "o"]);
    assert_eq!(floor_with(submit), Some(6), "above the floor it had, never 0");
    assert_eq!(floor_with(submit), Some(6), "and held");
}

/// The machine that refused every release: the old loader's self-test counted
/// every boot on the legacy counter 0x01000010, so the TPM's counters stood
/// high, and the TPM starts every new counter above that, through TPM2_Clear.
/// The floor is the counter's rise over its base, so it begins at 0 there.
#[test]
fn a_tpm_whose_counters_counted_starts_the_floor_at_zero_through_a_clear() {
    let Some(t) = Swtpm::start("a_tpm_whose_counters_counted_starts_the_floor_at_zero_through_a_clear") else {
        return;
    };
    let legacy = ["0x01000010"];
    tool(Some(&t), "tpm2_nvdefine", &[legacy[0], "-C", "o", "-s", "8", "-a", "authread|authwrite|nt=counter|no_da"]);
    for _ in 0..40 {
        tool(Some(&t), "tpm2_nvincrement", &[legacy[0], "-C", legacy[0]]);
    }
    /* Clear TPM, as the machine's owner did to boot again. */
    tool(Some(&t), "tpm2_clear", &["-c", "p"]);
    assert_eq!(floor_and_base_with(submit), Some((0, true)), "the counter starts above 40; the floor at 0");
    assert!(raise_with(submit, 1), "release 1 boots");
    assert_eq!(floor_with(submit), Some(1));
    tool(Some(&t), "tpm2_clear", &["-c", "p"]);
    assert_eq!(floor_and_base_with(submit), Some((0, true)), "after another clear, 0 again, and said");
    assert!(raise_with(submit, 1));
    assert_eq!(floor_with(submit), Some(1), "and held");
}

/// The base takes no write once set: only deleting it moves it.
#[test]
fn the_base_is_locked_once_written() {
    let Some(t) = Swtpm::start("the_base_is_locked_once_written") else {
        return;
    };
    assert!(raise_with(submit, 3));
    let out = tool(Some(&t), "tpm2_nvreadpublic", &["0x01000021"]);
    let text = String::from_utf8_lossy(&out);
    assert!(text.contains("writelocked"), "{text}");
    assert_eq!(floor_with(submit), Some(3));
}

/// The other R20 suggestion, POLICY_DELETE under an unsatisfiable policy, is
/// refused: an index the owner defines cannot carry it.
#[test]
fn an_owner_counter_cannot_carry_policy_delete() {
    let Some(_t) = Swtpm::start("an_owner_counter_cannot_carry_policy_delete") else {
        return;
    };
    let base = rb_define();
    let mut cmd = Vec::from(&base[..41]);
    cmd[37..41].copy_from_slice(&(0x0204_0014u32 | 0x400).to_be_bytes());
    cmd.extend_from_slice(&[0, 32]);
    cmd.extend_from_slice(&[0xFF; 32]);
    cmd.extend_from_slice(&base[43..]);
    cmd[29..31].copy_from_slice(&(14u16 + 32).to_be_bytes());
    let n = cmd.len() as u32;
    cmd[2..6].copy_from_slice(&n.to_be_bytes());
    let mut out = [0u8; 64];
    assert!(submit(&cmd, &mut out).is_some_and(|n| n >= 10));
    let rc = u32::from_be_bytes([out[6], out[7], out[8], out[9]]);
    assert_eq!(rc, 0x2C2, "TPM_RC_ATTRIBUTES on publicInfo");
}
