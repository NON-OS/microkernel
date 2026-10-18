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

//! get_primary: the surface the compositor maps, answered with what its
//! setup checks (userland/compositor/src/setup/prime_once.rs): format
//! ARGB8888, a stride of four bytes a pixel and a byte length of stride
//! times height, as the kernel registered them. The surface is the driver's
//! own memory; the framebuffer is never handed over.

use nonos_libc::surface_registry::SURFACE_SHARE_WRITE;
use nonos_libc::{registered, shares, Share, SHARED_HANDLE_BASE, SURFACE_FORMAT_ARGB8888};

use super::frames::ask;
use super::model::{served, COMPOSITOR, OTHER};
use crate::serve::{E_BUSY, OP_GET_PRIMARY_SURFACE, PRIMARY_RESOURCE_ID};

fn u32_at(b: &[u8], at: usize) -> u32 {
    u32::from_le_bytes(b[at..at + 4].try_into().expect("four bytes"))
}

#[test]
fn get_primary_hands_over_the_surface_the_kernel_registered() {
    let mut s = served(64, 48);
    let (status, reply) = ask(&mut s.d, COMPOSITOR, OP_GET_PRIMARY_SURFACE, &[]);
    assert_eq!((status, reply.len(), u32_at(&reply, 16)), (0, 56, 36), "status and a 32-byte body");
    let body = &reply[24..];
    let handle = u64::from_le_bytes(body[0..8].try_into().expect("eight bytes"));
    assert_eq!(handle, SHARED_HANDLE_BASE as u64, "not the handle the kernel shared");
    assert_eq!(u32_at(body, 8), PRIMARY_RESOURCE_ID);
    assert_eq!((u32_at(body, 12), u32_at(body, 16), u32_at(body, 20)), (64, 48, 256));
    assert_eq!((u32_at(body, 24), u32_at(body, 28)), (SURFACE_FORMAT_ARGB8888, 0));
    let desc = registered();
    assert_eq!(desc.len(), 1, "one surface registered");
    let d = desc[0];
    assert_eq!((d.width, d.height, d.stride, d.format), (64, 48, 256, SURFACE_FORMAT_ARGB8888));
    assert_eq!((d.byte_len, d.base_va), (256 * 48, s.a.surface.base()));
    assert_ne!(d.base_va, s.a.fb.base(), "the framebuffer was handed to the compositor");
}

#[test]
fn the_compositor_alone_is_named_the_surfaces_writable_receiver() {
    let mut s = served(64, 48);
    assert_eq!(shares(), [], "shared before the compositor asked");
    assert_eq!(ask(&mut s.d, OTHER, OP_GET_PRIMARY_SURFACE, &[]).0, E_BUSY);
    assert_eq!(shares(), [], "a capsule refused the primary was named its receiver");
    assert_eq!(ask(&mut s.d, COMPOSITOR, OP_GET_PRIMARY_SURFACE, &[]).0, 0);
    let named = Share { sid: 0, target_pid: COMPOSITOR, flags: SURFACE_SHARE_WRITE };
    assert_eq!(shares(), [named], "the compositor not named, writable");
}
