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

//! libc reads every field the kernel writes, and refuses a record whose fixed
//! bytes are not the kernel's.

use nonos_attest_path::Kind;

use super::record::{encode, RECORD_LEN, SLOT_LEN};
use super::slot::{slot, Slot};
use crate::fixture::{measure, trailer, tree};
use crate::libc_slots::{parse_boot_slots, BootSlot, BOOT_SLOTS_LEN, PATH_LEN};

fn slots() -> (Slot, Slot) {
    let (lm, km) = ([0x33; 32], measure(&[0x7F; 1000]));
    let (_, lt) = tree(Kind::Bootloader, &[lm]);
    let (_, kt) = tree(Kind::Kernel, &[km]);
    let b = slot(Kind::Bootloader, &lm, &trailer(&lt, Kind::Bootloader, 0)).expect("loader");
    let k = slot(Kind::Kernel, &km, &trailer(&kt, Kind::Kernel, 0)).expect("kernel");
    (b, k)
}

fn same(k: &Slot, l: &BootSlot) {
    let kernel = (k.kind, k.epoch, k.measurement, k.digest, k.root, k.path);
    assert_eq!(kernel, (l.kind, l.epoch, l.measurement, l.digest, l.root, l.path));
}

#[test]
fn libc_reads_what_the_kernel_writes() {
    assert_eq!(RECORD_LEN, BOOT_SLOTS_LEN);
    let (b, k) = slots();
    let got = parse_boot_slots(&encode(&b, &k)).expect("parsed");
    same(&b, &got.bootloader);
    same(&k, &got.kernel);
}

#[test]
fn every_fixed_byte_is_checked() {
    let (b, k) = slots();
    let r = encode(&b, &k);
    let mut fixed: Vec<usize> = (0..8).collect();
    for base in [8, 8 + SLOT_LEN] {
        fixed.extend(base..base + 8);
        fixed.extend(base + 112..base + 121);
        fixed.extend(base + 112 + PATH_LEN..base + SLOT_LEN);
    }
    for i in fixed {
        let mut g = r;
        g[i] ^= 0x01;
        assert_eq!(parse_boot_slots(&g), None, "byte {i}");
    }
}

#[test]
fn slots_out_of_order_or_another_length_are_refused() {
    let (b, k) = slots();
    assert_eq!(parse_boot_slots(&encode(&k, &b)), None);
    let r = encode(&b, &k);
    assert_eq!(parse_boot_slots(&r[..RECORD_LEN - 1]), None);
    assert_eq!(parse_boot_slots(&[&r[..], &[0]].concat()), None);
}
