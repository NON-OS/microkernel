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

//! Every table above, cut at every length and with every byte past the
//! checksum set to hostile values: none panics.

use super::hostile::every_cut_and_byte;
use super::iort_build::{iort_of, nodes};
use super::{
    gtdt_build as gtdt_tests, madt_build as madt_tests, pptt_build as pptt_tests, spcr_tests,
};
use crate::arm_acpi::{gtdt::gtdt, iort, madt::madt_gic, pptt::pptt, spcr::spcr};

#[test]
fn madt() {
    let t = madt_tests::madt(&[madt_tests::gicc(82, 0, 0, 1), madt_tests::gicc(76, 1, 1, 1)]);
    every_cut_and_byte(&t, madt_gic);
}

#[test]
fn gtdt_all_revisions() {
    for rev in [2, 3] {
        let t = gtdt_tests::table(rev, &[gtdt_tests::gt_block(2), gtdt_tests::watchdog()]);
        every_cut_and_byte(&t, gtdt);
    }
}

#[test]
fn spcr_all_revisions() {
    for rev in [1, 2, 3, 4] {
        every_cut_and_byte(&spcr_tests::table(rev, true, b"\\_SB.COM0\0"), spcr);
    }
}

#[test]
fn pptt_table() {
    every_cut_and_byte(&pptt_tests::table(), pptt);
}

#[test]
fn iort_table() {
    let t = iort_of(&nodes());
    every_cut_and_byte(&t, |b| iort::iort(b).map(|n| (iort::translate(&n, 160, 0x42), n.len())));
}
