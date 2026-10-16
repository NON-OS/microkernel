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

//! Every armed wake is undone when its holder's claim ends, on each path:
//! the claim released, the holder exited, the device departed. A model of
//! the kernel's claim test answers as the kernel does; a device still
//! held stays armed, and one claimed afresh by another pid does not.

use super::wake_model::{armed, swept};
use crate::wake_sweep::owns;

#[test]
fn nothing_is_disarmed_while_every_claim_holds() {
    let (k, owners) = armed();
    assert!(swept(&k, &owners).is_empty());
}

#[test]
fn each_way_a_claim_ends_disarms_exactly_its_devices() {
    let (k, owners) = armed();
    let mut released = k.clone();
    released.release(12);
    assert_eq!(swept(&released, &owners), [3], "released");
    let mut exited = k.clone();
    exited.exit(100);
    assert_eq!(swept(&exited, &owners), [2, 1], "exited: both, newest first");
    let mut departed = k.clone();
    departed.depart(13);
    assert_eq!(swept(&departed, &owners), [4], "departed");
    let mut taken = k.clone();
    taken.release(12);
    taken.0.get_mut(&12).unwrap().1 = Some(999);
    assert_eq!(swept(&taken, &owners), [3], "claimed afresh by another pid");
}

#[test]
fn a_device_the_kernel_resolves_elsewhere_is_disarmed() {
    let (mut k, owners) = armed();
    k.0.insert(10, (77, Some(100)));
    assert_eq!(swept(&k, &owners), [1], "a new arrival under the old device id");
}

#[test]
fn only_the_pid_that_armed_reads_the_count() {
    let (_, owners) = armed();
    assert!(owns(&owners, 1, 100) && owns(&owners, 3, 200));
    assert!(!owns(&owners, 1, 200) && !owns(&owners, 5, 100), "nothing armed by default");
}
