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

//! A capsule loaded at runtime takes no baked name (bga's README blocker):
//! the compositor's name, and with it the primaries every display driver
//! shares with whoever holds it, stays with the pid the kernel spawned
//! under it. Only a name the caller already holds on that port, or one of
//! the runtime-registrable net names with the register right, is allowed.

use super::register_allowed::allowed;
use crate::process::core::spawn_tier::set_system;
use crate::services::registry::{hold, set_right};
use crate::syscall::microkernel::errnos::ERRNO_PERM;

const COMPOSITOR: u32 = 7;
const INTRUDER: u32 = 66;

#[test]
fn no_capsule_takes_the_compositor_s_name_from_the_compositor() {
    hold("compositor", COMPOSITOR, 4310);
    set_right(true);
    assert_eq!(allowed("compositor", 4310, INTRUDER), Err(ERRNO_PERM));
    assert_eq!(allowed("compositor", 9999, INTRUDER), Err(ERRNO_PERM));
    assert_eq!(allowed("compositor", 4310, COMPOSITOR), Ok(()), "its own name again");
}

#[test]
fn nor_takes_it_while_it_is_free_even_with_the_register_right() {
    set_right(true);
    for name in ["compositor", "driver.bga0", "driver.virtio_gpu0", "wm"] {
        assert_eq!(allowed(name, 5000, INTRUDER), Err(ERRNO_PERM), "{name}");
    }
    assert_eq!(allowed("net.dns", 5000, INTRUDER), Ok(()), "a runtime net name");
    set_right(false);
    assert_eq!(allowed("net.dns", 5000, INTRUDER), Err(ERRNO_PERM), "and only with the right");
}

#[test]
fn a_publisher_s_capsule_takes_no_runtime_name_even_with_the_right() {
    set_right(true);
    set_system(false);
    let refused = allowed("net.dns", 5000, INTRUDER);
    set_system(true);
    assert_eq!(refused, Err(ERRNO_PERM), "only one spawned the system's own (L61)");
}
