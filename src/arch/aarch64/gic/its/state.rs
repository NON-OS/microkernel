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

//! The one ITS the kernel drives, behind a lock every caller takes: none
//! of it runs in interrupt context, and each operation is a few commands
//! with a bounded wait.

use spin::Mutex;

use super::super::intid::LPI_COUNT;
use super::super::msi::MsiError;
use super::encode::typer::ItsTyper;
use super::lpi_tables::LpiTables;
use super::memory::Region;
use super::queue::Queue;
use super::regs::Regs;

/// Devices with vectors at once; a capsule binds a handful.
pub(super) const MAX_DEVICES: usize = 64;
/// Events per device, the most MSI-X vectors one bind takes.
pub(super) const EVENT_BITS: u32 = 5;
/// The collection every LPI goes through, mapped to the boot CPU.
pub(super) const COLLECTION: u16 = 0;
pub(super) const LPI_WORDS: usize = (LPI_COUNT / 64) as usize;

#[derive(Clone, Copy)]
pub(super) struct Device {
    pub(super) id: u32,
    pub(super) itt: Region,
    /// Events in use, a bit each.
    pub(super) events: u32,
}

pub(super) struct Its {
    pub(super) regs: Regs,
    pub(super) queue: Queue,
    pub(super) typer: ItsTyper,
    pub(super) event_bits: u32,
    pub(super) device_limit: u64,
    pub(super) lpi: LpiTables,
    pub(super) devices: [Option<Device>; MAX_DEVICES],
    /// LPIs in use, a bit each from `LPI_BASE`.
    pub(super) lpis: [u64; LPI_WORDS],
}

pub(super) static ITS: Mutex<Option<Its>> = Mutex::new(None);

pub(super) fn with<R>(f: impl FnOnce(&mut Its) -> Result<R, MsiError>) -> Result<R, MsiError> {
    f(ITS.lock().as_mut().ok_or(MsiError::NotReady)?)
}
