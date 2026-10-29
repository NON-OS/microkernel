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

//! A stick boot never keeps the installed disk beside it: not when the
//! loader came through USB, nor when it named its partition and read the
//! store from that disk. The boot reads the loader's copy and keeps nothing.
//! A disk that may be the boot disk is still waited for, and otherwise the
//! other disk is kept as before. The loader's marks and the kernel's reads
//! of them are held to one value by their source (other_disk_usb_tests.rs
//! too); glue_locks_tests holds settle.rs to asking this rule.

use super::other_disk::{other_disk, Other};
use crate::proofs::proofs_8::transport_takeover::text::code;

const BOOTED: &str = include_str!("../../../../../src/hardware/block_device/booted.rs");
const KERNEL: &str = include_str!("../../../../../src/boot/handoff/types/boot_evidence.rs");
const LOADER: &str = include_str!("../../../../../nonos-bootloader/src/handoff/types/system.rs");
const SETS: &str = include_str!("../../../../../nonos-bootloader/src/entry/boot_disk.rs");

#[test]
fn never_a_different_disk_when_the_loader_named_its_own() {
    assert_eq!(other_disk(true, true, false, false, false), Other::Leave, "no device");
    assert_eq!(other_disk(true, true, false, true, true), Other::Leave, "the wait ran out");
}

#[test]
fn a_disk_that_may_be_the_boot_disk_is_still_waited_for() {
    for bits in 0..8u8 {
        let [named, own, usb] = [0, 1, 2].map(|i| bits >> i & 1 == 1);
        assert_eq!(other_disk(named, own, usb, true, false), Other::Wait, "{bits:03b}");
    }
}

#[test]
fn without_the_boot_disks_own_store_the_other_is_kept_as_before() {
    // A loader on a disk with no store, a firmware that read another's, an older loader.
    assert_eq!(other_disk(true, false, false, false, false), Other::Keep);
    assert_eq!(other_disk(true, false, false, true, true), Other::Keep);
    // No partition named: the store copy cannot say which disk is the boot's.
    assert_eq!(other_disk(false, true, false, false, false), Other::Keep);
}

#[test]
fn every_case() {
    for bits in 0..32u8 {
        let [named, own, usb, refused, waited] = [0, 1, 2, 3, 4].map(|i| bits >> i & 1 == 1);
        let (wait, leave) = (refused && !waited, (usb || named && own) && (!refused || waited));
        let got = other_disk(named, own, usb, refused, waited);
        assert_eq!((got == Other::Wait, got == Other::Leave), (wait, leave), "{bits:05b}");
    }
}

#[test]
fn the_loaders_mark_is_the_one_the_kernel_reads() {
    let flag = code("pub const STORE_FROM_BOOT_DISK: u32 = 1;");
    assert!(code(KERNEL).contains(&flag) && code(LOADER).contains(&flag), "one value");
    let set = code("let reserved = if own { STORE_FROM_BOOT_DISK } else { 0 };");
    assert!(code(SETS).contains(&set), "set when the copy came from the loader's disk");
    let b = code(BOOTED);
    assert!(b.contains(&code(".is_some_and(|m| m.reserved & STORE_FROM_BOOT_DISK != 0);")));
    assert!(b.contains(&code("marked && super::store_copy::present()")), "a copy that answers");
}
