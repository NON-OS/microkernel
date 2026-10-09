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

//! The rollback counter's three commands, its base's four, and what a read
//! answered. Pure, so the host proofs hold the mapping to each response code.

use super::consts::{response_code, DEFINE_COUNTER_CMD, INCREMENT_CMD, READ_CMD, RC_NV_UNINITIALIZED};

/// The counter is 0x01000020; the templates name the legacy 0x01000010.
const INDEX_LOW: u8 = 0x20;

pub fn rb_define() -> [u8; 45] {
    let mut cmd = DEFINE_COUNTER_CMD;
    cmd[34] = INDEX_LOW;
    cmd
}

pub fn rb_increment() -> [u8; 31] {
    let mut cmd = INCREMENT_CMD;
    cmd[13] = INDEX_LOW;
    cmd[17] = INDEX_LOW;
    cmd
}

/// The base, 0x01000021: the counter's value when this machine's floor began.
const BASE_LOW: u8 = 0x21;

/// An ordinary index of 8 bytes, written once and then locked until it is
/// deleted: no DA, auth read, write define, auth write.
pub fn base_define() -> [u8; 45] {
    let mut cmd = DEFINE_COUNTER_CMD;
    cmd[34] = BASE_LOW;
    cmd[37..41].copy_from_slice(&0x0204_2004u32.to_be_bytes());
    cmd
}

pub fn base_read() -> [u8; 35] {
    let mut cmd = READ_CMD;
    cmd[13] = BASE_LOW;
    cmd[17] = BASE_LOW;
    cmd
}

/// TPM2_NV_Write of `value` at offset 0, the index authorizing with an empty
/// password. TPM 2.0 Part 3, 31.7.
pub fn base_write(value: u64) -> [u8; 43] {
    let mut cmd = [0u8; 43];
    cmd[..10].copy_from_slice(&[0x80, 0x02, 0, 0, 0, 43, 0, 0, 0x01, 0x37]);
    cmd[10..14].copy_from_slice(&[0x01, 0, 0, BASE_LOW]);
    cmd[14..18].copy_from_slice(&[0x01, 0, 0, BASE_LOW]);
    cmd[18..22].copy_from_slice(&9u32.to_be_bytes());
    cmd[22..26].copy_from_slice(&0x4000_0009u32.to_be_bytes());
    cmd[31..33].copy_from_slice(&8u16.to_be_bytes());
    cmd[33..41].copy_from_slice(&value.to_be_bytes());
    cmd
}

/// TPM2_NV_WriteLock: no write reaches the base again until it is deleted.
/// TPM 2.0 Part 3, 31.11.
pub fn base_lock() -> [u8; 31] {
    let mut cmd = INCREMENT_CMD;
    cmd[9] = 0x38;
    cmd[13] = BASE_LOW;
    cmd[17] = BASE_LOW;
    cmd
}

pub fn rb_read() -> [u8; 35] {
    let mut cmd = READ_CMD;
    cmd[13] = INDEX_LOW;
    cmd[17] = INDEX_LOW;
    cmd
}

/// What a TPM2_NV_Read of the counter answered.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FloorRead {
    Value(u64),
    /// Defined and never incremented, TPM_RC_NV_UNINITIALIZED: a counter just
    /// defined, on a new TPM or after one was deleted.
    Uninitialized,
    Unreadable,
}

/// The first `n` bytes of `resp`: on success, parameter size 10, then the
/// counter as a TPM2B of 8 bytes, then the password session's answer.
pub fn floor_read(resp: &[u8], n: usize) -> FloorRead {
    let Some(r) = resp.get(..n).filter(|r| r.len() >= 10) else {
        return FloorRead::Unreadable;
    };
    match response_code(r) {
        RC_NV_UNINITIALIZED => FloorRead::Uninitialized,
        0 if r.len() >= 24 && r[10..16] == [0, 0, 0, 10, 0, 8] => {
            let mut v = [0u8; 8];
            v.copy_from_slice(&r[16..24]);
            FloorRead::Value(u64::from_be_bytes(v))
        }
        _ => FloorRead::Unreadable,
    }
}

/// An increment, or any command without output, succeeded.
pub fn succeeded(resp: &[u8], n: usize) -> bool {
    resp.get(..n).is_some_and(|r| r.len() >= 10 && response_code(r) == 0)
}
