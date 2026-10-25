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

//! Every ECAM window the platform described, keyed on (segment, bus).
//!
//! Append-only: a window is published once, at boot, and never moves, so a
//! config access finds its window with plain loads and no lock. Producers are
//! the x86 MCFG, the aarch64 device tree and each VMD domain.

use core::sync::atomic::{AtomicUsize, Ordering};

use spin::Mutex;

use super::error::EcamError;
use super::slot::Slot;
use super::window::EcamWindow;

/// Room for a server's MCFG segment groups and a domain per VMD.
const SLOTS: usize = 32;

static TABLE: [Slot; SLOTS] = [const { Slot::new() }; SLOTS];
static COUNT: AtomicUsize = AtomicUsize::new(0);
/// Serialises producers only. Readers never take it.
static WRITER: Mutex<()> = Mutex::new(());

/// Refuses a window that maps nothing or claims a bus another one claims:
/// two windows for one bus would make the route depend on the search order.
pub fn publish(w: EcamWindow) -> Result<(), EcamError> {
    if w.is_empty() {
        return Err(EcamError::Empty);
    }
    let _guard = WRITER.lock();
    let n = COUNT.load(Ordering::Relaxed);
    if published(n).any(|old| old.overlaps(&w)) {
        return Err(EcamError::Overlaps);
    }
    let slot = TABLE.get(n).ok_or(EcamError::Full)?;
    slot.store(w);
    COUNT.store(n + 1, Ordering::Release);
    Ok(())
}

pub fn lookup(segment: u16, bus: u8) -> Option<EcamWindow> {
    current().find(|w| w.covers(segment, bus))
}

/// Each segment that has a window, once, in the order first published.
pub fn segments() -> impl Iterator<Item = u16> {
    let n = COUNT.load(Ordering::Acquire);
    (0..n).filter_map(move |i| {
        let seg = TABLE[i].load().segment;
        (!published(i).any(|w| w.segment == seg)).then_some(seg)
    })
}

fn current() -> impl Iterator<Item = EcamWindow> {
    published(COUNT.load(Ordering::Acquire))
}

fn published(n: usize) -> impl Iterator<Item = EcamWindow> {
    TABLE[..n].iter().map(Slot::load)
}
