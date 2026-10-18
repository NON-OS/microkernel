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

//! libc's `IrqPark` when the kernel refuses its grant: the shim's
//! `mk_irq_wait` answers as such a kernel does.

use nonos_libc::wait::{IrqPark, SPIN_BUDGET};
use nonos_libc::{slept_ms, yields};

#[test]
fn a_refused_grant_sleeps_that_step_and_is_dropped() {
    let park = IrqPark::new();
    park.set_grant(7);
    let (yielded, slept) = (yields(), slept_ms());
    park.park_step(SPIN_BUDGET + 1);
    assert_eq!(yields(), yielded, "it tried the interrupt, not a yield");
    assert!(slept_ms() > slept, "and slept instead of spinning straight back");
    park.park_step(SPIN_BUDGET + 1);
    assert_eq!(yields(), yielded + 1, "polling without the grant from now on");
}

#[test]
fn each_park_keeps_its_own_grant() {
    let (refused, untouched) = (IrqPark::new(), IrqPark::new());
    refused.set_grant(7);
    refused.park_step(SPIN_BUDGET + 1);
    let (yielded, slept) = (yields(), slept_ms());
    untouched.park_step(SPIN_BUDGET + 1);
    assert_eq!(yields(), yielded + 1, "a park with no grant yields; it never had one");
    assert_eq!(slept_ms(), slept);
}
