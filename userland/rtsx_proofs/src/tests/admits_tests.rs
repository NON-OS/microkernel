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

//! The medium held to the kernel's block layer and the installer (C6), and
//! each answered where it waits.

use nonos_libc::{grant_cap, To};

use super::rig::{sdhc, serving};
use super::service_rig::{call, span, INSTALLER, KERNEL, STORE_WRITE};
use crate::protocol::*;
use crate::serve::admits::allows;

#[test]
fn only_the_kernel_and_a_store_write_holder_reach_the_medium() {
    let (mut drv, mut medium) = serving(sdhc(1));
    for op in [OP_CAPACITY, OP_READ, OP_WRITE, OP_FLUSH, OP_IDENTIFY] {
        let body = if op == OP_READ { span(0, 1) } else { vec![] };
        assert_eq!(call(&mut drv, &mut medium, INSTALLER, op, &body).status, E_ACCES);
    }
    assert_eq!(call(&mut drv, &mut medium, INSTALLER, OP_HEALTHCHECK, &[]).status, E_OK);
    grant_cap(INSTALLER, STORE_WRITE);
    assert_eq!(call(&mut drv, &mut medium, INSTALLER, OP_READ, &span(0, 1)).status, E_OK);
    assert_eq!(call(&mut drv, &mut medium, KERNEL, OP_READ, &span(0, 1)).status, E_OK);
    assert!(allows(OP_HEALTHCHECK, 9, false) && allows(OP_WRITE, 0, false));
    assert!(!allows(OP_WRITE, 9, false) && allows(OP_WRITE, 9, true));
}

#[test]
fn the_kernel_is_answered_on_its_reply_endpoint_and_a_capsule_on_its_call() {
    let (mut drv, mut medium) = serving(sdhc(1));
    let kernel = call(&mut drv, &mut medium, KERNEL, OP_HEALTHCHECK, &[]);
    assert_eq!(kernel.to, To::Endpoint(KERNEL_REPLY_ENDPOINT));
    assert_eq!(KERNEL_REPLY_ENDPOINT, 4_294_967_380, "Capsule.mk's reply endpoint");
    let capsule = call(&mut drv, &mut medium, INSTALLER, OP_HEALTHCHECK, &[]);
    assert_eq!(capsule.to, To::Pid(INSTALLER));
}
