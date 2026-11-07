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

/*
 * Which of a guest's pages its supervisor may pin as a surface.
 *
 * A pinned surface is the guest's own frames, registered in the guest's name
 * and shared read-only, so the compositor reads a Linux window's pixels
 * where the client drew them instead of from a copy. Only pages the guest
 * has mapped for itself now may be pinned, none executable, whole pages from
 * a page boundary: a frame the guest cannot reach holds none of its pixels,
 * and an executable one is code. Pure, so kernel_proofs::surface_pin holds
 * it by path; pin_guest.rs reads the guest's tables and the registry.
 */

extern crate alloc;

use alloc::vec::Vec;

pub const PAGE: u64 = 4096;

/// Surfaces the guests of one supervisor hold at once, pinned or kept for a
/// holder after an unpin. The 256 slots are the machine's: one supervisor's
/// guests must leave the rest for every other window, however many guests
/// it hosts.
pub const PINS_PER_SUPERVISOR: usize = 16;

/// The endpoint a pin's reader serves: the compositor.
pub const READER: &str = "compositor";

/// One page of the guest as its tables map it now.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GuestPage {
    pub frame: u64,
    /// The guest may touch it: clear for a page mapped with PROT_NONE.
    pub user: bool,
    pub exec: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinRefusal {
    /// Not a whole run of pages from a page boundary in the user half, or
    /// rows that do not fit it.
    Shape,
    /// A page the guest has no mapping of, or none it may touch.
    Unmapped,
    /// A page the guest may run as code.
    Executable,
    /// Part of the range is a broker window or a DMA grant.
    DeviceWindow,
    /// A frame backs another surface, or the range maps one frame twice.
    AlreadyHeld,
    /// The supervisor's guests hold PINS_PER_SUPERVISOR surfaces already.
    Full,
    /// A reader that is not the compositor, or a scanout reader that serves
    /// no display driver's endpoint.
    Reader,
}

/// The shape a pin names: `height` rows of `stride` bytes, each `width`
/// pixels of four bytes, in `byte_len` bytes at guest address `addr`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PinShape {
    pub addr: u64,
    pub byte_len: u64,
    pub width: u32,
    pub height: u32,
    pub stride: u32,
}

/// How many pages `shape` covers, when it is a pin at all: from a page
/// boundary, ending at or below `user_end`, its rows inside its bytes, and
/// at most `max_pages` pages.
pub fn pin_pages(shape: PinShape, user_end: u64, max_pages: usize) -> Result<usize, PinRefusal> {
    let PinShape { addr, byte_len, width, height, stride } = shape;
    if addr % PAGE != 0 || byte_len == 0 || width == 0 || height == 0 {
        return Err(PinRefusal::Shape);
    }
    let row = width.checked_mul(4).ok_or(PinRefusal::Shape)?;
    let rows = u64::from(stride).checked_mul(u64::from(height)).ok_or(PinRefusal::Shape)?;
    if stride < row || rows > byte_len {
        return Err(PinRefusal::Shape);
    }
    let end = addr.checked_add(byte_len).ok_or(PinRefusal::Shape)?;
    if end > user_end {
        return Err(PinRefusal::Shape);
    }
    let pages = usize::try_from(byte_len.div_ceil(PAGE)).map_err(|_| PinRefusal::Shape)?;
    if pages > max_pages {
        return Err(PinRefusal::Shape);
    }
    Ok(pages)
}

/// The frames of `pages`, in page order, when every page is mapped for the
/// guest and none is executable.
pub fn pin_frames(pages: &[Option<GuestPage>]) -> Result<Vec<u64>, PinRefusal> {
    let mut frames = Vec::with_capacity(pages.len());
    for page in pages {
        let page = page.ok_or(PinRefusal::Unmapped)?;
        if !page.user {
            return Err(PinRefusal::Unmapped);
        }
        if page.exec {
            return Err(PinRefusal::Executable);
        }
        frames.push(page.frame);
    }
    Ok(frames)
}

/// Whether a frame lies in one of `grants`, each a DMA run as (physical
/// start, length): a device reaches it, so no surface may hold it.
pub fn in_grant(frames: &[u64], grants: impl IntoIterator<Item = (u64, u64)>) -> bool {
    grants.into_iter().any(|(start, len)| {
        let end = start.saturating_add(len);
        frames.iter().any(|&f| f >= start && f < end)
    })
}

/// Room for one more surface, when the supervisor's guests hold `owned`.
pub const fn room(owned: usize) -> bool {
    owned < PINS_PER_SUPERVISOR
}

/// Whether a process serving the endpoints `served` may be named to read a
/// pin: the compositor as its reader, or, as its scanout reader, a display
/// driver of `displays` (services/registry/displays.rs). Nobody else: the
/// guest's pixels go where they are shown, and nowhere more.
pub fn may_read(served: &[&str], displays: &[&str], scanout: bool) -> bool {
    match scanout {
        false => served.contains(&READER),
        true => served.iter().any(|name| displays.contains(name)),
    }
}
