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


//! The session volume's sectors, held in RAM, on a boot whose disk carries
//! the NONOS store but no data plan: a live stick. Nothing of it reaches a
//! disk, and it is gone at power off. What it holds is sealed like any
//! other volume's sectors, under a key made for this boot alone.
//!
//! Frames are taken as sectors are first written, never all at once, and a
//! sector never written reads as zeros, as a blank disk's does. A frame
//! holds 8 sectors; a leaf frame holds the addresses of 512 of them, so 2
//! MiB of sectors; the top table names the leaves.

use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, Ordering};

use spin::Mutex;

use crate::hardware::block_device::BlockDeviceError;
use crate::memory::addr::PhysAddr;
use crate::memory::phys::{AllocFlags, PAGE_SIZE_U64};

const SECTOR: usize = 512;
const PER_FRAME: u64 = PAGE_SIZE_U64 / SECTOR as u64;
const PER_LEAF: u64 = PER_FRAME * (PAGE_SIZE_U64 / 8);

struct Ram {
    sectors: u64,
    /// Each leaf frame's physical address, 0 for none yet.
    leaves: Vec<u64>,
}

static RAM: Mutex<Option<Ram>> = Mutex::new(None);
static ON: AtomicBool = AtomicBool::new(false);

/// Whether the volume's sectors are in RAM this boot.
pub fn on() -> bool {
    ON.load(Ordering::SeqCst)
}

/// What the rest of the machine keeps: the larger of 1 GiB and a quarter
/// of its memory. The volume grows only while more than this is free.
pub fn reserve() -> u64 {
    (crate::memory::phys::total_memory() / 4).max(1 << 30)
}

/// Whether the volume may take more memory now. Asked once per 2 MiB of new
/// blocks, since counting what is free walks the whole frame map.
pub fn room() -> bool {
    crate::memory::phys::free_memory() > reserve()
}

/// Sectors per check of `room`.
pub const ROOM_EVERY: u64 = PER_LEAF;

/// Hold `sectors` in RAM from now on. Once a boot: the volume it backs is
/// the one this boot has.
pub fn start(sectors: u64) -> Result<(), BlockDeviceError> {
    let mut ram = RAM.lock();
    if ram.is_some() {
        return Err(BlockDeviceError::InvalidArgument);
    }
    let count = usize::try_from(sectors.div_ceil(PER_LEAF)).map_err(|_| BlockDeviceError::OutOfRange)?;
    let mut leaves = Vec::new();
    leaves.try_reserve_exact(count).map_err(|_| BlockDeviceError::OversizedRequest)?;
    leaves.resize(count, 0);
    *ram = Some(Ram { sectors, leaves });
    ON.store(true, Ordering::SeqCst);
    Ok(())
}

pub fn read(lba: u64, out: &mut [u8]) -> Result<(), BlockDeviceError> {
    let guard = RAM.lock();
    let ram = guard.as_ref().ok_or(BlockDeviceError::Dead)?;
    for (i, sector) in out.chunks_mut(SECTOR).enumerate() {
        let at = ram.check(lba, i, sector.len())?;
        match ram.frame(at) {
            Some(frame) => load(frame, at, sector)?,
            None => sector.fill(0),
        }
    }
    Ok(())
}

pub fn write(lba: u64, bytes: &[u8]) -> Result<(), BlockDeviceError> {
    let mut guard = RAM.lock();
    let ram = guard.as_mut().ok_or(BlockDeviceError::Dead)?;
    for (i, sector) in bytes.chunks(SECTOR).enumerate() {
        let at = ram.check(lba, i, sector.len())?;
        let frame = ram.frame_or_take(at)?;
        store(frame, at, sector)?;
    }
    Ok(())
}

impl Ram {
    fn check(&self, lba: u64, i: usize, len: usize) -> Result<u64, BlockDeviceError> {
        let at = lba.checked_add(i as u64).ok_or(BlockDeviceError::OutOfRange)?;
        if at >= self.sectors || len != SECTOR {
            return Err(BlockDeviceError::OutOfRange);
        }
        Ok(at)
    }

    fn frame(&self, at: u64) -> Option<u64> {
        let leaf = self.leaves[(at / PER_LEAF) as usize];
        if leaf == 0 {
            return None;
        }
        match slot(leaf, ((at % PER_LEAF) / PER_FRAME) as usize) {
            0 => None,
            frame => Some(frame),
        }
    }

    fn frame_or_take(&mut self, at: u64) -> Result<u64, BlockDeviceError> {
        let top = (at / PER_LEAF) as usize;
        if self.leaves[top] == 0 {
            self.leaves[top] = take()?;
        }
        let (leaf, i) = (self.leaves[top], ((at % PER_LEAF) / PER_FRAME) as usize);
        match slot(leaf, i) {
            0 => {
                let frame = take()?;
                set_slot(leaf, i, frame);
                Ok(frame)
            }
            frame => Ok(frame),
        }
    }
}

/// A zeroed frame the direct map reaches, or a refusal when the machine has
/// none to give.
fn take() -> Result<u64, BlockDeviceError> {
    let frame = crate::memory::phys::alloc(AllocFlags::ZERO).ok_or(BlockDeviceError::DeviceFailure)?;
    if base(frame.0).is_none() {
        let _ = crate::memory::phys::free(frame);
        return Err(BlockDeviceError::DeviceFailure);
    }
    Ok(frame.0)
}

/// Where `frame` is in the direct map.
fn base(frame: u64) -> Option<*mut u8> {
    crate::memory::unified::phys_to_virt(PhysAddr::new(frame)).map(|v| v.as_u64() as *mut u8)
}

/*
 * SAFETY for every access below: eK@nonos.systems - each frame was taken by
 * this module, checked to be in the direct map, is never given back, and is
 * touched only under the RAM lock; every offset stays inside its 4 KiB.
 */

fn slot(leaf: u64, i: usize) -> u64 {
    match base(leaf) {
        Some(p) => unsafe { core::ptr::read((p as *const u64).add(i)) },
        None => 0,
    }
}

fn set_slot(leaf: u64, i: usize, frame: u64) {
    if let Some(p) = base(leaf) {
        unsafe { core::ptr::write((p as *mut u64).add(i), frame) };
    }
}

fn load(frame: u64, at: u64, out: &mut [u8]) -> Result<(), BlockDeviceError> {
    let p = base(frame).ok_or(BlockDeviceError::DeviceFailure)?;
    let from = ((at % PER_FRAME) as usize) * SECTOR;
    unsafe { core::ptr::copy_nonoverlapping(p.add(from), out.as_mut_ptr(), SECTOR) };
    Ok(())
}

fn store(frame: u64, at: u64, bytes: &[u8]) -> Result<(), BlockDeviceError> {
    let p = base(frame).ok_or(BlockDeviceError::DeviceFailure)?;
    let from = ((at % PER_FRAME) as usize) * SECTOR;
    unsafe { core::ptr::copy_nonoverlapping(bytes.as_ptr(), p.add(from), SECTOR) };
    Ok(())
}
