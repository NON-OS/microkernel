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

//! A device that is not VMMDev, or a host that will not take the guest:
//! everything given back, and integration never turned on, so the PS/2
//! pointer keeps moving.

use nonos_libc::calls;

use super::util::up_with;
use crate::model::Options;

fn given_back(opt: Options) -> Vec<u32> {
    let (d, host) = up_with(opt);
    assert!(d.is_err(), "the bring-up must refuse");
    let got = calls();
    let back = [("dma_unmap", 70), ("mmio_unmap", 60), ("pio_release", 50), ("device_release", 4)];
    assert_eq!(got[got.len() - 4..], back);
    let host = host.borrow();
    assert_eq!(host.guest_mouse, 0, "integration never turned on");
    host.types()
}

#[test]
fn an_event_page_that_is_not_vmmdev_memory_is_refused() {
    for (memory_version, memory_size) in [(2, 0x1000), (1, 16), (1, 0x0080_0000)] {
        let opt = Options { memory_version, memory_size, ..Options::default() };
        assert!(given_back(opt).is_empty(), "no request before the page checks out");
    }
}

#[test]
fn a_host_that_never_answers_is_refused() {
    let types = given_back(Options { deaf: true, ..Options::default() });
    assert!(types.is_empty());
}

#[test]
fn a_host_that_refuses_the_guest_info_is_refused() {
    let types = given_back(Options { refuse_info: true, ..Options::default() });
    assert_eq!(types, vec![4, 50]);
}
