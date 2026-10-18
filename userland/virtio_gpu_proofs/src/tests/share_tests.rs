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

//! OP_GET_PRIMARY_SURFACE names its asker the surface's one receiver,
//! writable: the kernel lets only a process a share names attach the
//! surface, and the client draws the screen into it. A second asker is
//! refused before any share, so the kernel never names it.

use nonos_libc::shares;
use nonos_libc::surface_registry::SURFACE_SHARE_WRITE;

use super::client::call;
use super::stall::{handle, CLIENT, PRIMARY};
use super::{driver::driver, harness::harness};
use crate::protocol::{E_BUSY, OP_GET_PRIMARY_SURFACE};
use crate::setup::resource;
use crate::state::Scanout;

#[test]
fn get_primary_names_its_client_the_writable_receiver_and_no_one_else() {
    let (d, _region, _regs) = driver(harness(), Some(PRIMARY));
    let (width, height) = (PRIMARY.width, PRIMARY.height);
    let whole = Scanout { x: 0, y: 0, width, height, current_resource_id: 0, enabled: true };
    resource::insert(&d.resources, PRIMARY.resource_id, whole).expect("room");
    assert_eq!(shares(), [], "shared before anyone asked");
    assert_eq!(handle(&d), Some(PRIMARY.handle), "the client not handed the surface");
    assert_eq!(shares(), [(PRIMARY.sid, CLIENT, SURFACE_SHARE_WRITE)], "not named, writable");
    assert_eq!(call(&d, CLIENT + 1, OP_GET_PRIMARY_SURFACE, &[]), E_BUSY, "a second owner");
    assert_eq!(shares().len(), 1, "the kernel told to let a second process attach it");
}
