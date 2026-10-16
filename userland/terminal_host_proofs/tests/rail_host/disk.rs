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

//! Store usage reply decoding.

use crate::rail::disk::Disk;
use crate::rail::disk_decode::{decode_usage, REPLY_LEN as DISK_REPLY_LEN};
use crate::rail::net_decode::HDR_LEN;
use crate::rail::value::Metric;

fn usage(status: i32, bytes: u64) -> [u8; DISK_REPLY_LEN] {
    let mut rx = [0u8; DISK_REPLY_LEN];
    rx[HDR_LEN..HDR_LEN + 4].copy_from_slice(&status.to_le_bytes());
    rx[HDR_LEN + 4..HDR_LEN + 8].copy_from_slice(&3u32.to_le_bytes());
    rx[HDR_LEN + 8..HDR_LEN + 16].copy_from_slice(&bytes.to_le_bytes());
    rx[HDR_LEN + 16..HDR_LEN + 20].copy_from_slice(&2048u32.to_le_bytes());
    rx
}

#[test]
fn a_usage_reply_yields_the_bytes_the_store_holds() {
    let d = decode_usage(&usage(0, 5 * 1024 * 1024));
    assert_eq!(d.used_kb, Metric::Known(5 * 1024));
    assert!(d.total_kb == Metric::Unsupported);
}

#[test]
fn an_empty_store_is_a_measured_zero_rather_than_a_gap() {
    assert_eq!(decode_usage(&usage(0, 0)).used_kb, Metric::Known(0));
}

#[test]
fn a_failed_or_short_usage_reply_leaves_the_store_unmeasured() {
    assert_eq!(decode_usage(&usage(-13, 4096)), Disk::UNKNOWN);
    assert_eq!(decode_usage(&usage(0, 4096)[..DISK_REPLY_LEN - 1]), Disk::UNKNOWN);
}
