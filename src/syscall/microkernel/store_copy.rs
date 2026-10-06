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

//! The package store as the loader read it, through the firmware's own disk
//! driver.
//!
//! A machine whose USB stick, controller or disk the kernel's drivers do not
//! bring up still has its store: the loader read it whole into memory the
//! kernel never reclaims, and `MkStoreRead` answers from that copy. Store
//! writes land in the copy as well as on the disk, so the two agree for the
//! boot. When a disk is kept and its store does not start as the copy does
//! (the loader came from another disk), the copy is set aside and the disk
//! answers, as before, so reads and writes stay on one store.

use core::sync::atomic::{AtomicBool, AtomicU8, Ordering};

use spin::{Mutex, Once};

use crate::boot::handoff::types::MODULE_KIND_STORE;
use crate::memory::addr::PhysAddr;
use crate::memory::unified::phys_to_virt;

use super::store_copy_span::{held, offset, SECTOR, STORE_BASE_LBA};

/* The header and the table of contents, compared with a kept disk's. */
const HEAD_SECTORS: u64 = 128;
const MAGIC: &[u8; 8] = b"NONOSTR1";

struct Copy {
    at: u64,
    len: usize,
}

static COPY: Once<Option<Copy>> = Once::new();
static LOCK: Mutex<()> = Mutex::new(());

const UNCHECKED: u8 = 0;
const AGREES: u8 = 1;
const SET_ASIDE: u8 = 2;
static STATE: AtomicU8 = AtomicU8::new(UNCHECKED);
/* One caller compares at a time; the others answer from the copy meanwhile. */
static CHECKING: AtomicBool = AtomicBool::new(false);
/* Comparisons a disk that would not be read has failed. */
static FAILED_CHECKS: AtomicU8 = AtomicU8::new(0);
/*
 * A kept disk that cannot be read this many times is no longer asked: each
 * failed read of a USB stick can take its twelve seconds, and asking on every
 * store read would stall the whole store behind it. The copy goes on
 * answering; writes still go to the disk.
 */
const MAX_FAILED_CHECKS: u8 = 3;
const GAVE_UP: u8 = 3;

fn copy() -> Option<&'static Copy> {
    COPY.call_once(|| {
        let m = super::install_source_modules::find(MODULE_KIND_STORE as u64)?;
        let len = held(usize::try_from(m.size).ok()?);
        let last = m.base.checked_add(len as u64 - 1)?;
        phys_to_virt(PhysAddr::new(last))?;
        let at = phys_to_virt(PhysAddr::new(m.base))?.as_u64();
        // SAFETY: eK@nonos.systems - the loader read whole pages into loader
        // memory, which the kernel never frees, so the bytes up to the end of
        // the last sector are there; both ends were just checked to lie under
        // the directmap.
        let head = unsafe { core::slice::from_raw_parts(at as *const u8, len.min(8)) };
        if head != MAGIC {
            return None;
        }
        let line = alloc::format!("[STORE] the loader's copy of the store: {} bytes", len);
        crate::sys::serial::println(line.as_bytes());
        Some(Copy { at, len })
    })
    .as_ref()
}

/* The copy's bytes for sectors `lba..` of length `len`, when it holds all
 * of them and has not been set aside. */
fn span(lba: u64, len: usize) -> Option<&'static mut [u8]> {
    if STATE.load(Ordering::Acquire) == SET_ASIDE {
        return None;
    }
    let c = copy()?;
    let off = offset(lba, len, c.len)?;
    // SAFETY: eK@nonos.systems - in bounds of the loader's copy, checked
    // above; every access to it holds LOCK.
    Some(unsafe { core::slice::from_raw_parts_mut((c.at as usize + off) as *mut u8, len) })
}

/// Whether the loader handed over a copy of the store: this boot came from a
/// NONOS disk, whether or not a driver of the kernel's drives it.
pub fn present() -> bool {
    copy().is_some()
}

/// Fill `out` with sectors from `lba` from the copy; false when the copy
/// does not hold them, and the disk is asked instead.
pub fn read(lba: u64, out: &mut [u8]) -> bool {
    check_against_disk();
    let _held = LOCK.lock();
    match span(lba, out.len()) {
        Some(bytes) => {
            out.copy_from_slice(bytes);
            true
        }
        None => false,
    }
}

/// Sectors `bytes` just written from `lba` to the disk, kept in the copy too.
pub fn wrote(lba: u64, bytes: &[u8]) {
    let _held = LOCK.lock();
    for (i, sector) in bytes.chunks(SECTOR).enumerate() {
        if let Some(dst) = span(lba + i as u64, sector.len()) {
            dst.copy_from_slice(sector);
        }
    }
}

/// Once a disk is kept, whether its store starts as the copy does; the copy
/// is set aside when it does not. A disk that cannot be read now is asked
/// again on a later read, up to `MAX_FAILED_CHECKS` times.
pub fn check_against_disk() {
    if STATE.load(Ordering::Acquire) != UNCHECKED
        || crate::hardware::block_device::chosen().is_none()
        || CHECKING.swap(true, Ordering::AcqRel)
    {
        return;
    }
    compare();
    CHECKING.store(false, Ordering::Release);
}

fn compare() {
    let Some(c) = copy() else { return };
    let len = c.len.min(HEAD_SECTORS as usize * SECTOR);
    let mut disk = alloc::vec![0u8; len.div_ceil(SECTOR) * SECTOR];
    for (i, chunk) in disk.chunks_mut(64 * SECTOR).enumerate() {
        let lba = STORE_BASE_LBA + (i * 64) as u64;
        if crate::hardware::block_device::read(lba, chunk).is_err() {
            if FAILED_CHECKS.fetch_add(1, Ordering::AcqRel) + 1 >= MAX_FAILED_CHECKS {
                STATE.store(GAVE_UP, Ordering::Release);
                crate::sys::serial::println(
                    b"[STORE] the kept disk would not be read; the loader's copy answers",
                );
            }
            return;
        }
    }
    let _held = LOCK.lock();
    let Some(mine) = span(STORE_BASE_LBA, len) else { return };
    let (state, line): (u8, &[u8]) = if mine[..] == disk[..len] {
        (AGREES, b"[STORE] the kept disk carries the store the loader read")
    } else {
        (SET_ASIDE, b"[STORE] the kept disk carries another store; the loader's copy is set aside")
    };
    STATE.store(state, Ordering::Release);
    crate::sys::serial::println(line);
}
