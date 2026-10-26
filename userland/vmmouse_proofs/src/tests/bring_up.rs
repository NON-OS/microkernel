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

//! The version, the order of the bring-up calls, and every host on which
//! there is no pointer.

use nonos_libc::{backdoor_calls, backdoor_present, set_host, Hypervisor};

use super::util::host;
use crate::constants::{DISABLE, ENABLE, REQUEST_ABSOLUTE};
use crate::model::{Host, VMWARE};
use crate::setup::{enable, version};

#[test]
fn version_then_off_on_identity_and_absolute_in_order() {
    let h = host();
    assert_eq!(version(), Some(6));
    assert_eq!(enable(), Ok(()));
    assert_eq!(
        backdoor_calls(),
        [(10, 0), (41, DISABLE), (41, ENABLE), (40, 0), (39, 1), (41, REQUEST_ABSOLUTE)]
    );
    let s = h.0.borrow();
    assert!(s.absolute && s.queue.is_empty() && s.status == 0);
}

#[test]
fn kvm_and_tcg_reach_qemu_s_vmport() {
    for vendor in [Hypervisor::Kvm, Hypervisor::QemuTcg] {
        Host::new().install(vendor as u8);
        assert_eq!((version(), backdoor_present()), (Some(6), true), "{}", vendor.slug());
    }
}

#[test]
fn no_other_host_is_asked() {
    assert_eq!(version(), None, "no host at all");
    for vendor in [0, 4, 5, 6, 7, 8, 9, 10].map(Hypervisor::from_code) {
        Host::new().install(vendor as u8);
        assert_eq!((version(), backdoor_present()), (None, false), "{}", vendor.slug());
        assert_eq!(enable(), Err("pointer command refused"), "{}", vendor.slug());
    }
    assert!(backdoor_calls().is_empty(), "the kernel refused every call");
}

#[test]
fn a_port_that_does_not_answer_is_no_backdoor() {
    let ones = ([u32::MAX; 4], "all ones");
    let no_magic = ([6, 0, 0, 0], "no magic back");
    for (regs, why) in [ones, no_magic, ([u32::MAX, 0x564D_5868, 0, 0], "the magic, no version")] {
        set_host(VMWARE, Box::new(move |_| regs));
        assert_eq!((version(), backdoor_present()), (None, false), "{why}: nor to the kernel");
    }
}

#[test]
fn a_vmport_without_the_pointer_is_left_off() {
    let h = host();
    h.0.borrow_mut().pointer = false;
    assert_eq!(enable(), Err("not the absolute pointer"));
    assert_eq!(backdoor_calls().last(), Some(&(41, DISABLE)));
}

#[test]
fn a_pointer_that_names_another_is_turned_off() {
    let h = host();
    h.0.borrow_mut().wrong_id = true;
    assert_eq!(enable(), Err("not the absolute pointer"));
    assert_eq!(backdoor_calls().last(), Some(&(41, DISABLE)));
    assert_eq!(h.0.borrow().status, 0xFFFF);
}

#[test]
fn a_pointer_that_queues_nothing_is_not_there() {
    let h = host();
    h.0.borrow_mut().dead = true;
    assert_eq!(enable(), Err("no absolute pointer"));
    assert!(!h.0.borrow().absolute);
}

#[test]
fn a_pointer_still_in_error_is_not_there() {
    let h = host();
    h.0.borrow_mut().enable_keeps_error = true;
    assert_eq!(enable(), Err("no absolute pointer"));
    assert!(!h.0.borrow().absolute);
}

#[test]
fn a_pointer_left_on_with_packets_queued_is_reset_first() {
    let h = host();
    {
        let mut s = h.0.borrow_mut();
        s.status = 0;
        s.absolute = true;
    }
    h.mouse(100, 200, 1, 0);
    assert_eq!(enable(), Ok(()), "the old packet is not taken for the identity");
}
