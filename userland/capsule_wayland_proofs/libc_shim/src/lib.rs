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

//! The calls the Wayland server makes into the kernel, answered on the host.
//! What each one did is kept per thread (world.rs), so a test or the host
//! server reads back what the server said, registered and was sent.

mod mem;
pub mod world;

pub use world::SurfaceDescriptor;

/// The router's event as libc lays it out, for its delivery decoder.
pub mod types {
    #[derive(Clone, Copy, Default)]
    pub struct InputEvent {
        pub kind: u16,
        pub flags: u16,
        pub code: u32,
        pub x: i32,
        pub y: i32,
        pub delta_x: i32,
        pub delta_y: i32,
        pub timestamp_ns: u64,
    }
}

/// The router's delivery frame, read by the real decoder.
#[path = "../../../libc/src/surface_registry/delivery.rs"]
pub mod delivery;

pub use delivery::{
    decode_delivery, delivery_window, Delivered, DELIVERY_FLAG_WINDOW, DELIVERY_HDR_LEN, DELIVERY_LEN, DELIVERY_LEN_V1,
    DELIVERY_MAGIC, DELIVERY_VERSION,
};

/// The surface registry's ARGB8888, which display_info checks for.
pub const SURFACE_FORMAT_ARGB8888: u32 = 1;

/// One line of the server's debug output, kept and echoed to stderr.
pub fn mk_debug(buf: *const u8, len: usize) -> i64 {
    if buf.is_null() || len == 0 {
        return -22;
    }
    // SAFETY: every caller passes a live slice's pointer and length.
    let line = unsafe { core::slice::from_raw_parts(buf, len) };
    world::say(line);
    len as i64
}

/// Anonymous pages (mem.rs); a failure is a negative errno as the pointer.
pub extern "C" fn mk_mmap(
    _addr: *mut u8,
    len: usize,
    _prot: i32,
    _flags: i32,
    _fd: i32,
    _offset: i64,
) -> *mut u8 {
    mem::map(len) as *mut u8
}

pub extern "C" fn mk_munmap(addr: *mut u8, len: usize) -> i64 {
    mem::unmap(addr as usize, len)
}

pub extern "C" fn mk_surface_register(desc: *const SurfaceDescriptor) -> i64 {
    if desc.is_null() {
        return -22;
    }
    // SAFETY: the server passes a reference to a descriptor it holds.
    world::register(unsafe { *desc })
}

pub extern "C" fn mk_surface_share(sid: u64, _target_pid: u32, _flags: u64) -> i64 {
    world::share(sid)
}

pub extern "C" fn mk_surface_release(handle: u64) -> i64 {
    world::release(handle)
}

/// The release count of surface `sid`, the caller's own. The host does not
/// wait: the compositor double releases in the same thread (desk.rs).
pub extern "C" fn mk_surface_fence_wait(sid: u64, _seen: u32, timeout_ms: u64) -> i64 {
    world::fence_wait(sid, timeout_ms)
}

/// The next frame the host put in the inbox, as the kernel's receive gives
/// it: its length, or -11 (EAGAIN) when none waits.
pub extern "C" fn mk_ipc_recv_from(
    _endpoint: u64,
    buf: *mut u8,
    len: usize,
    _timeout_ms: u64,
    sender_pid_out: *mut u32,
) -> i64 {
    let Some((sender, frame)) = world::next_frame() else { return -11 };
    let n = frame.len().min(len);
    // SAFETY: `buf` is the caller's buffer of `len` bytes; the sender slot
    // is the caller's u32.
    unsafe {
        core::ptr::copy_nonoverlapping(frame.as_ptr(), buf, n);
        if !sender_pid_out.is_null() {
            *sender_pid_out = sender;
        }
    }
    n as i64
}

/// The pin calls (MkPeerPin, MkPeerUnpin): the host kernel double pins by
/// registering the guest's span as a surface, so a pin's handle is a
/// surface's here and its fence is that surface's (world.rs).
pub mod peer {
    use super::world::{self, SurfaceDescriptor};

    pub fn mk_peer_pin(_pid: u32, desc: &SurfaceDescriptor, reader: u32, scanout: u32) -> i64 {
        world::pin(*desc, (reader, scanout))
    }

    pub fn mk_peer_unpin(_pid: u32, handle: u64) -> i64 {
        world::unpin(handle)
    }

    /// A surface's handle is its sid here (world.rs share).
    pub fn pin_sid(handle: u64) -> u64 {
        handle
    }
}
