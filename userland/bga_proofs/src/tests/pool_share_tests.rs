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

//! K20 on the display framework's surface pool, which bga and virtio-gpu
//! register and share each output through (nonos_gfx_pool): the share names
//! the compositor, writable, and no one else; with no compositor running
//! nothing is registered or shared; a withdrawn output gives its handle back.

#[path = "../../../nonos_gfx_pool/src/surface.rs"]
mod pool_surface;

use nonos_gfx_serve::Region;
use nonos_gfx_wire::errno::EINVAL;
use nonos_libc::surface_registry::SURFACE_SHARE_WRITE;
use nonos_libc::{hold, registered, released, shares, Share, SHARED_HANDLE_BASE};

use pool_surface::{publish, withdraw};

const COMPOSITOR: u32 = 42;

fn output() -> Region {
    Region { base: 0x4000_0000, len: 640 * 480 * 4 }
}

#[test]
fn the_pool_names_the_compositor_alone_as_the_writable_receiver() {
    hold(b"compositor", Some(COMPOSITOR));
    let handle = publish(output(), 640, 480, 640 * 4).expect("shared");
    assert_eq!(shares(), [Share { sid: 0, target_pid: COMPOSITOR, flags: SURFACE_SHARE_WRITE }]);
    assert_eq!(handle, SHARED_HANDLE_BASE as u64, "the share's handle, never the surface id");
    assert_eq!((registered()[0].width, registered()[0].byte_len), (640, 640 * 480 * 4));
}

#[test]
fn with_no_compositor_nothing_is_registered_or_shared() {
    hold(b"compositor", None);
    assert_eq!(publish(output(), 640, 480, 640 * 4), Err(EINVAL));
    assert!(registered().is_empty() && shares().is_empty());
}

#[test]
fn a_withdrawn_output_gives_its_share_back() {
    hold(b"compositor", Some(COMPOSITOR));
    let handle = publish(output(), 640, 480, 640 * 4).expect("shared");
    withdraw(handle);
    assert_eq!(released(), [handle]);
}
