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

use core::sync::atomic::{AtomicU32, AtomicU64};
use spin::Mutex;

/// `0` is the sentinel for "kernel half" or "no asid scoping". A
/// flush issued with `asid == ASID_KERNEL` reaches every online CPU
/// because the kernel half is shared across every address space.
pub const ASID_KERNEL: u32 = 0;

/// Hard bound on the cross-CPU wait, in wall-clock milliseconds, converted to
/// ticks against the calibrated counter frequency by `wait::budget`. A healthy
/// ack takes microseconds, but a peer vCPU descheduled by its host, or one
/// finishing a stretch of work between two serve points under emulation, can
/// take tens of milliseconds, and halting the machine over a slow peer is
/// worse than waiting for it. Tuned upwards is fine; tuned to "wait forever"
/// is forbidden, because a peer that never answers is a real defect.
pub(super) const SHOOTDOWN_TIMEOUT_MS: u64 = 2000;

/// An ack later than this is reported, once per round, and waited for.
pub(super) const SHOOTDOWN_WARN_MS: u64 = 50;

/// Tick budget used when the computed budget comes back `0` (uncalibrated,
/// or a frequency too low to clear one millisecond at this resolution). At
/// least 2000ms on any CPU up to 5 GHz.
pub(super) const SHOOTDOWN_TIMEOUT_FALLBACK_TICKS: u64 = 10_000_000_000;

/// The same for the warning: at least 50ms on any CPU up to 5 GHz.
pub(super) const SHOOTDOWN_WARN_FALLBACK_TICKS: u64 = 250_000_000;

pub(super) static SHOOTDOWN_LOCK: Mutex<()> = Mutex::new(());
pub(super) static REQ_VA: AtomicU64 = AtomicU64::new(0);
pub(super) static REQ_PAGES: AtomicU32 = AtomicU32::new(0);
pub(super) static REQ_PENDING_ACKS: AtomicU32 = AtomicU32::new(0);
