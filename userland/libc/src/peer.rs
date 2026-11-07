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

//! Reaching into a guest this process supervises: its pages, their protection,
//! and the bytes in them.

use crate::surface_registry::SurfaceDescriptor;
use crate::syscall::{
    call_raw, N_MK_PEER_COPY, N_MK_PEER_MAP, N_MK_PEER_PIN, N_MK_PEER_PROTECT, N_MK_PEER_READ_ROWS,
    N_MK_PEER_TLS, N_MK_PEER_UNMAP, N_MK_PEER_UNPIN,
};

/// Pages of a guest may be written, and may be executed.
pub const PEER_PROT_WRITE: u64 = 1 << 0;
pub const PEER_PROT_EXEC: u64 = 1 << 1;
/// Pages the guest may not touch at all, their bytes kept for a later
/// protection that opens them: PROT_NONE.
pub const PEER_PROT_NONE: u64 = 1 << 2;

/// Back a span of a guest's address space with fresh zeroed frames.
pub fn mk_peer_map(pid: u32, addr: u64, len: u64, prot: u64) -> i64 {
    call_raw(N_MK_PEER_MAP, [pid as u64, addr, len, prot, 0, 0])
}

/// Set the protection of pages a guest already has.
pub fn mk_peer_protect(pid: u32, addr: u64, len: u64, prot: u64) -> i64 {
    call_raw(N_MK_PEER_PROTECT, [pid as u64, addr, len, prot, 0, 0])
}

/// Take a span back from a guest. What exec needs, so a replacing
/// image cannot see the pages of the one it replaced.
pub fn mk_peer_unmap(pid: u32, addr: u64, len: u64) -> i64 {
    call_raw(N_MK_PEER_UNMAP, [pid as u64, addr, len, 0, 0, 0])
}

/// The thread pointer a guest thread wakes with. A C runtime sets this
/// during startup and dereferences it immediately afterwards.
pub fn mk_peer_tls(pid: u32, base: u64) -> i64 {
    call_raw(N_MK_PEER_TLS, [pid as u64, base, 0, 0, 0, 0])
}

/// Copy into a guest this process supervises.
pub fn mk_peer_write(pid: u32, guest_addr: u64, src: &[u8]) -> i64 {
    let args = [pid as u64, guest_addr, src.as_ptr() as u64, src.len() as u64, 1, 0];
    call_raw(N_MK_PEER_COPY, args)
}

/// Copy out of a guest this process supervises.
pub fn mk_peer_read(pid: u32, guest_addr: u64, dst: &mut [u8]) -> i64 {
    let args = [pid as u64, guest_addr, dst.as_mut_ptr() as u64, dst.len() as u64, 0, 0];
    call_raw(N_MK_PEER_COPY, args)
}

/// Copy `count` rows of `row` bytes out of a guest this process supervises,
/// row `i` from `guest_addr + i * stride` to `dst[i * stride..]`, in one
/// call; the bytes of `dst` between the rows are left as they are. Refused
/// unless the rows fit `dst`, a row fits its stride, and there are at most
/// 4096 rows of at most 1 MiB in all (src/process/foreign/peer_rows_shape.rs).
pub fn mk_peer_read_rows(
    pid: u32,
    guest_addr: u64,
    dst: &mut [u8],
    row: usize,
    count: usize,
    stride: usize,
) -> i64 {
    let span =
        count.checked_sub(1).and_then(|n| stride.checked_mul(n)).and_then(|s| s.checked_add(row));
    if span.is_none_or(|end| end > dst.len()) {
        return -22;
    }
    let args =
        [pid as u64, guest_addr, dst.as_mut_ptr() as u64, row as u64, count as u64, stride as u64];
    call_raw(N_MK_PEER_READ_ROWS, args)
}

/// Pin the pages of a guest this process supervises that `desc` names, its
/// `base_va` a guest address on a page boundary, as a surface the guest's
/// group owns, shared read-only with `reader`, the compositor, and, when
/// not zero, `scanout`, a display driver. Returns the surface's handle, or
/// a negative errno: EINVAL for a shape or a page not mapped for the guest,
/// EPERM for a reader that is neither, an executable page or a frame
/// another surface or a device holds, ENOMEM when this process's guests
/// hold 16 surfaces already (src/kernel_core/surface_registry/pin/pin_rule.rs).
pub fn mk_peer_pin(pid: u32, desc: &SurfaceDescriptor, reader: u32, scanout: u32) -> i64 {
    let desc = desc as *const SurfaceDescriptor as u64;
    call_raw(N_MK_PEER_PIN, [pid as u64, desc, reader as u64, scanout as u64, 0, 0])
}

/// Give up a surface `mk_peer_pin` made for guest `pid`: nobody attaches it
/// again; a view already mapped keeps its frames until that reader lets go.
pub fn mk_peer_unpin(pid: u32, handle: u64) -> i64 {
    call_raw(N_MK_PEER_UNPIN, [pid as u64, handle, 0, 0, 0, 0])
}

/// The sid MkSurfaceFenceWait names a pinned surface by: its slot, the
/// handle's high word (src/kernel_core/surface_registry/handle.rs).
pub fn pin_sid(handle: u64) -> u64 {
    handle >> 32
}
