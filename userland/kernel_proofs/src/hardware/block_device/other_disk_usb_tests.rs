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

//! A stick boot's mark, from the loader's device path to the kernel's rule:
//! the loader records BOOT_MEDIA_USB whenever its volume path has a USB
//! node, named partition or not, and takes no other disk's store then; the
//! kernel reads the same bit and settle.rs hands it to other_disk. On an HP
//! laptop the loader's same-disk test missed the stick, its store was
//! marked another disk's, and the installed NVMe disk was kept.

use crate::proofs::proofs_8::transport_takeover::text::code;

const KERNEL: &str = include_str!("../../../../../src/boot/handoff/types/boot_media.rs");
const LOADER: &str =
    include_str!("../../../../../nonos-bootloader/src/handoff/types/boot_media.rs");
const SETS: &str = include_str!("../../../../../nonos-bootloader/src/entry/install_source.rs");
const STORE: &str = include_str!("../../../../../nonos-bootloader/src/loader/file/store_copy.rs");
const BOOTED: &str = include_str!("../../../../../src/hardware/block_device/booted.rs");
const SETTLE: &str = include_str!("../../../../../src/hardware/block_device/settle.rs");

#[test]
fn the_usb_bit_is_one_value_on_both_sides() {
    let bit = code("pub const BOOT_MEDIA_USB: u16 = 1;");
    assert!(code(KERNEL).contains(&bit) && code(LOADER).contains(&bit), "one value");
    for side in [KERNEL, LOADER] {
        assert!(code(side).contains(&code("pub flags: u16,")), "at bytes 6..8");
    }
}

#[test]
fn the_loader_marks_every_usb_boot_named_or_not() {
    let s = code(SETS);
    assert!(s.contains(&code("let usb = boot_device(bs) == BootDevice::UsbStick;")));
    assert!(s.contains(&code("boot_partition(bs).or_else(|| usb.then(BootMedia::default))")));
    assert!(s.contains(&code("flags: if usb { BOOT_MEDIA_USB } else { 0 }, ..record")));
}

#[test]
fn a_stick_takes_no_other_disks_store() {
    let s = code(STORE);
    assert!(s.contains(&code("let stick = boot_device(bs) == BootDevice::UsbStick;")));
    assert!(
        s.contains(&code("let rest = handles.iter().zip(&ours).filter(|(_, &o)| !o && !stick);"))
    );
    assert!(s.contains(&code("first.chain(rest).find_map(")), "its own disk first, then the rest");
}

#[test]
fn the_kernel_reads_the_bit_and_the_rule_is_asked_with_it() {
    let b = code(BOOTED);
    assert!(b.contains(&code("u16::from_le_bytes([r[6], r[7]]) & BOOT_MEDIA_USB != 0")));
    let s = code(SETTLE);
    let seen = "(partition().is_some(), store_from_boot_disk(), from_usb());";
    assert!(s.contains(&code(seen)), "each read once");
    assert!(s.contains(&code("other_disk(named, own_copy, usb, refused.is_some(), waited);")));
}
