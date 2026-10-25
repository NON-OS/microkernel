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
//! A mount takes the newest header a commit wrote, passes over slots that
//! hold none, and fails on a slot the device could not read, so an unread
//! newest header never mounts an older generation in its place.

use super::ring_pick::{pick, SlotRead};

const RING: u64 = 256;

fn header(generation: u64) -> SlotRead<u64, &'static str> {
    SlotRead::Header { at: generation % RING, generation, header: generation }
}

#[test]
fn the_newest_header_is_mounted_wherever_it_sits() {
    let slots = || [3, 300, 299, 7].map(header);
    assert_eq!(pick(slots(), RING), Ok(Some(300)));
    let mut reversed = slots();
    reversed.reverse();
    assert_eq!(pick(reversed, RING), Ok(Some(300)));
}

#[test]
fn a_device_failure_fails_the_mount_instead_of_an_older_header() {
    let slots = [header(5), SlotRead::Failed("device"), header(4)];
    assert_eq!(pick(slots, RING), Err("device"));
    let first = [SlotRead::Failed("first"), header(9), SlotRead::Failed("second")];
    assert_eq!(pick(first, RING), Err("first"));
}

#[test]
fn slots_with_no_header_are_passed_over() {
    let slots = [SlotRead::NotOurs, header(2), SlotRead::NotOurs];
    assert_eq!(pick(slots, RING), Ok(Some(2)));
    let none: [SlotRead<u64, &str>; 3] = [SlotRead::NotOurs, SlotRead::NotOurs, SlotRead::NotOurs];
    assert_eq!(pick(none, RING), Ok(None), "an unformatted ring");
}

#[test]
fn a_header_outside_its_generations_slot_is_not_a_commits() {
    let moved = SlotRead::Header { at: 1, generation: 900, header: 900 };
    assert_eq!(pick([header(3), moved], RING), Ok(Some(3)));
    let none = SlotRead::<u64, &str>::Header { at: 0, generation: 0, header: 0 };
    assert_eq!(pick([none], 0), Ok(None), "no ring, no header, no panic");
}

#[test]
fn the_kernel_mounts_through_the_rule() {
    let mount = include_str!("../../../../src/fs/blockfs/mount.rs");
    assert!(mount.contains("pick(slots, HEADER_RING_SECTORS)?.ok_or(BlockFsError::NotFormatted)"));
    assert!(mount.contains("Err(e) if not_ours(&e) => SlotRead::NotOurs,"));
    assert!(mount.contains("Err(e) => SlotRead::Failed(e),"));
    let ours = "BlockFsError::CryptoBlock(CryptoBlockError::AuthenticationFailed)\n            | BlockFsError::InvalidSuperblock\n    )";
    assert!(mount.contains(ours), "only a slot that holds no header is passed over");
    let lba = include_str!("../../../../src/fs/blockfs/header_lba.rs");
    assert!(lba.contains("generation % HEADER_RING_SECTORS"), "where a commit puts a header");
}
