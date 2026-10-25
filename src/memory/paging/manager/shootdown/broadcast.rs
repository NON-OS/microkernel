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

use super::round::round;
use crate::memory::addr::VirtAddr;

pub(super) fn broadcast(va: VirtAddr, page_count: u32, asid: u32) {
    /*
     * The whole round runs masked. A holder of `SHOOTDOWN_LOCK` that took an
     * interrupt could be switched away, or could reach a flush from the
     * handler, and every other cpu, this one's next thread included, would
     * spin on the lock with interrupts masked waiting for a round that no
     * longer runs. The wait itself serves this cpu's own requests by hand,
     * so masking costs it nothing it needs to hear.
     */
    let done = crate::arch::run_without_interrupts(|| round(va, page_count, asid));
    super::summary::after(done);
}
