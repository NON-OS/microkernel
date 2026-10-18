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

//! Devices that fail the driver: an error that names the step, the device
//! quiesced and reset before any grant goes back, and every grant returned.

use super::refuse::refused;
use super::util::commands;

const ACTIVATE: u32 = 0xCAFE_0000;

// The card never goes on the air under any address when none can be drawn.
#[test]
fn no_entropy_and_the_device_is_never_activated() {
    nonos_libc::entropy(false);
    let (err, d) = refused(|_| {});
    assert_eq!(err, "no entropy for station address");
    assert!(!commands(&d.log).contains(&ACTIVATE));
    assert_eq!(d.mac, [0; 6], "no address was ever set");
}

#[test]
fn a_host_without_revision_1_is_refused_before_anything_is_written() {
    let (err, d) = refused(|d| d.faults.no_rev1 = true);
    assert_eq!(err, "revision 1 not offered");
    assert_eq!((d.vrrs, d.uvrs), (0, 0));
    assert!(!commands(&d.log).contains(&ACTIVATE));
}

#[test]
fn a_refused_activation_clears_the_shared_address() {
    let (err, d) = refused(|d| d.faults.refuse_activation = true);
    assert_eq!(err, "device refused activation");
    assert!(!d.active);
    assert_eq!(d.dsa, 0);
}
