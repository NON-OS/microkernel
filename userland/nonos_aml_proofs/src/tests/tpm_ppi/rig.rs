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

//! A TPM device whose PPI _DSM lists functions 0, 1, 2 and 8 (the `mask`
//! a test gives), answers version "1.3", and for function 8 answers 3 for
//! Clear (5), 4 for operation 23, 1 for 14 and 0 otherwise. Function 2,
//! the request submission, counts its calls in \SUBM, which \GSUB reads.

use crate::enc::*;
use crate::tpm_ppi::PPI_UUID;

pub fn tpm(hid: &str, sta: u64, mask: &[u8]) -> Vec<u8> {
    let f = |n: u64, body: &[u8]| if_(&e(0x93, &[&arg(2), &int(n)]), body);
    let op = e(0x83, &[&e(0x88, &[&arg(3), &int(0), &[0x00]])]);
    let is_op = |n: u64, answer: u64| if_(&e(0x93, &[&local(0), &int(n)]), &ret(&int(answer)));
    let f8 =
        cat(&[&store(&op, &local(0)), &is_op(5, 3), &is_op(23, 4), &is_op(14, 1), &ret(&int(0))]);
    let submit = cat(&[
        &store(&e(0x72, &[&name("\\SUBM"), &int(1), &[0x00]]), &name("\\SUBM")),
        &ret(&int(0)),
    ]);
    let ppi = cat(&[
        &f(0, &ret(&buffer(mask.len() as u64, mask))),
        &f(1, &ret(&string("1.3"))),
        &f(2, &submit),
        &f(8, &f8),
    ]);
    let dsm =
        cat(&[&if_(&e(0x93, &[&arg(0), &buffer(16, &PPI_UUID)]), &ppi), &ret(&buffer(1, &[0]))]);
    let body = cat(&[
        &name_obj("_HID", &string(hid)),
        &name_obj("_STA", &int(sta)),
        &method("_DSM", 4, &dsm),
    ]);
    cat(&[
        &name_obj("\\SUBM", &int(0)),
        &method("\\GSUB", 0, &ret(&name("\\SUBM"))),
        &device("\\_SB.TPM0", &body),
    ])
}
