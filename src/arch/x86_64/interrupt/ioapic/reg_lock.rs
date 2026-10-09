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
 * The IOAPIC is reached through an index register and a data window, so each
 * access is two MMIO operations and updating a redirection entry takes four
 * behind a read of both halves. Two CPUs interleaving them read or write the
 * wrong register. With the broker masking a line in its interrupt on one CPU
 * while a driver acknowledged on another, the virtio-blk entry was found with
 * its low word copied into its high word and its remote IRR stuck set, and
 * the line never fired again. One lock covers every access, taken with
 * interrupts masked because the broker takes it from interrupt context.
 */
static REG_LOCK: spin::Mutex<()> = spin::Mutex::new(());

/// Run `f` holding the IOAPIC register lock with interrupts masked. Every
/// index/window access to any IOAPIC in the kernel goes through here.
pub(crate) fn locked<R>(f: impl FnOnce() -> R) -> R {
    crate::arch::run_without_interrupts(|| {
        let _guard = REG_LOCK.lock();
        f()
    })
}
