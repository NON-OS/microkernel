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

//! The disk the store calls reach, one per test thread so proofs running in
//! parallel never see each other's sectors. Sectors never written read as
//! zeros, as a fresh disk's do.

use std::cell::RefCell;
use std::collections::BTreeMap;

pub const SECTOR: usize = 512;

/// Every sector that holds something, by LBA. Cloned to keep a disk as it
/// stood before a write and put it back before the next attempt.
#[derive(Clone, Default, PartialEq, Eq, Debug)]
pub struct Image(BTreeMap<u64, [u8; SECTOR]>);

pub(crate) struct Disk {
    pub(crate) image: Image,
    /// False when no disk carries NONOS: the kernel has selected nothing.
    pub(crate) present: bool,
    /// False when reads come from the loader's copy of the store and the
    /// kernel drives no disk to write it to.
    pub(crate) writable: bool,
    /// The errno every read answers with, for a device that faults.
    pub(crate) read_errno: Option<i64>,
    /// Sectors that still land before the power goes. `None` while it stays.
    pub(crate) power: Option<usize>,
    /// Sectors written that reached the medium.
    pub(crate) landed: usize,
    /// A multi-sector write lands all but its last sector and says so, as a
    /// device that took less than it was handed would.
    pub(crate) short_writes: bool,
}

impl Disk {
    const fn new() -> Disk {
        Disk {
            image: Image(BTreeMap::new()),
            present: true,
            writable: true,
            read_errno: None,
            power: None,
            landed: 0,
            short_writes: false,
        }
    }

    pub(crate) fn sector(&self, lba: u64) -> [u8; SECTOR] {
        self.image.0.get(&lba).copied().unwrap_or([0u8; SECTOR])
    }

    /// One sector of a write. Past the power cut it never reaches the
    /// medium; the caller cannot tell, as a machine losing power cannot.
    pub(crate) fn land(&mut self, lba: u64, bytes: &[u8]) {
        match self.power {
            Some(0) => return,
            Some(ref mut left) => *left -= 1,
            None => {}
        }
        let mut sector = [0u8; SECTOR];
        sector.copy_from_slice(bytes);
        self.image.0.insert(lba, sector);
        self.landed += 1;
    }
}

thread_local! {
    pub(crate) static DISK: RefCell<Disk> = const { RefCell::new(Disk::new()) };
}

/// A blank disk that is present, answers every read and keeps its power.
pub fn reset() {
    DISK.with(|d| *d.borrow_mut() = Disk::new());
}

/// No disk carries the store, as on a live boot with nothing installed.
pub fn remove() {
    DISK.with(|d| d.borrow_mut().present = false);
}

/// The loader's copy of the store answers every read and no disk the
/// kernel drives takes a write, as on a machine whose stick only the
/// firmware could read.
pub fn copy_only() {
    DISK.with(|d| d.borrow_mut().writable = false);
}

/// Every read from now on is refused with `errno`.
pub fn fail_reads(errno: i64) {
    DISK.with(|d| d.borrow_mut().read_errno = Some(errno));
}

/// Every multi-sector write from now on is cut one sector short.
pub fn short_writes() {
    DISK.with(|d| d.borrow_mut().short_writes = true);
}

/// The power goes after `sectors` more sectors land.
pub fn power_fails_after(sectors: usize) {
    DISK.with(|d| d.borrow_mut().power = Some(sectors));
}

/// The power is back: writes land again.
pub fn power_on() {
    DISK.with(|d| d.borrow_mut().power = None);
}

/// Sectors written so far that reached the medium.
pub fn landed() -> usize {
    DISK.with(|d| d.borrow().landed)
}

/// Put `bytes` on the disk from `lba`, the last sector zero padded.
pub fn put(lba: u64, bytes: &[u8]) {
    DISK.with(|d| {
        let mut d = d.borrow_mut();
        for (i, chunk) in bytes.chunks(SECTOR).enumerate() {
            let mut sector = [0u8; SECTOR];
            sector[..chunk.len()].copy_from_slice(chunk);
            d.image.0.insert(lba + i as u64, sector);
        }
    });
}

/// `sectors` sectors from `lba`.
pub fn get(lba: u64, sectors: usize) -> Vec<u8> {
    DISK.with(|d| {
        let d = d.borrow();
        (0..sectors as u64).flat_map(|i| d.sector(lba + i)).collect()
    })
}

/// The disk's contents as they stand.
pub fn image() -> Image {
    DISK.with(|d| d.borrow().image.clone())
}

/// Put the contents back as `image` held them, power on, count from zero.
pub fn restore(image: &Image) {
    DISK.with(|d| {
        let mut d = d.borrow_mut();
        d.image = image.clone();
        d.power = None;
        d.landed = 0;
    });
}
