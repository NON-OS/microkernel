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

//! Bring-up's primary surface: registered and shared with no one, and
//! the driver asks the kernel for no pid. The one share is get_primary's,
//! naming its client writable, and the client is handed the handle that
//! share answers.

use nonos_libc::surface_registry::{SHARED_HANDLE_BASE, SURFACE_SHARE_WRITE};
use nonos_libc::{registered, shares, FIRST_SID};

use super::part::{answering, Answer};
use super::stall::{handle, CLIENT};
use super::{driver::driver, harness::harness};
use crate::primary_surface::create::create;
use crate::state::Scanout;

#[test]
fn bring_up_shares_the_surface_with_no_one_and_the_client_gets_its_share() {
    let (mut d, region, _regs) = driver(harness(), None);
    let scanout =
        Scanout { x: 0, y: 0, width: 64, height: 32, current_resource_id: 0, enabled: true };
    let part = answering(&region, Answer::Spec);
    let made =
        create(d.device_id, d.claim_epoch, &d.control_queue, &d.fences, &d.resources, scanout);
    let primary = made.expect("brought up").expect("a surface");
    drop(part);
    assert_eq!(shares(), [], "shared at bring-up");
    let descs = registered();
    assert_eq!(descs.len(), 1, "one surface registered");
    assert_eq!((descs[0].width, descs[0].height, descs[0].stride), (64, 32, 256));
    assert_eq!((descs[0].owner_pid, descs[0].writable), (0, 0), "the attach's to write");
    assert_eq!(primary.sid, FIRST_SID as u64, "not the sid the kernel answered");
    d.primary = Some(primary);
    let answered = (SHARED_HANDLE_BASE + FIRST_SID) as u64;
    assert_eq!(handle(&d), Some(answered), "not the handle the client's share answers");
    assert_eq!(shares(), [(primary.sid, CLIENT, SURFACE_SHARE_WRITE)], "not named, writable");
}
