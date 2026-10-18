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
use super::park::Armed;
use crate::constants::{EVENT_RING_SEGMENT_TRBS, TRB_BYTES};
use crate::controller::park::IrqWait;
use crate::dma::{DmaPool, DmaRegion};
use crate::error::XhciResult;
/// Interrupt-IN TRBs that may be armed at once, across every device: four
/// endpoints on each of sixteen.
pub(super) const ARMED: usize = 64;
const ERST_ENTRY_BYTES: u64 = 16;
pub struct EventRing {
    pub(super) segment: DmaRegion,
    pub(super) erst: DmaRegion,
    pub(super) consumer_cycle: u8,
    pub(super) dequeue_index: usize,
    pub(super) drained_total: u64,
    /// Each armed TRB, and its event once a wait other than its own poll
    /// read it (`park`).
    pub(super) armed: [Option<Armed>; ARMED],
    /// The interrupt of the interrupter this ring belongs to, which its
    /// waits block on between polls.
    pub irq: IrqWait,
    /// The operational registers of the controller the ring was given to,
    /// 0 before `bring_up`; its waits look there for a failed controller.
    pub(super) op_base: u64,
}
impl EventRing {
    pub fn new(pool: &DmaPool) -> XhciResult<Self> {
        let segment_bytes = (EVENT_RING_SEGMENT_TRBS as u64) * (TRB_BYTES as u64);
        let segment = pool.alloc(segment_bytes)?;
        segment.zero();
        let erst = pool.alloc(ERST_ENTRY_BYTES)?;
        erst.zero();
        let segment_phys = segment.phys();
        // The one entry: segment base, then its size in TRBs.
        let entry =
            [segment_phys as u32, (segment_phys >> 32) as u32, EVENT_RING_SEGMENT_TRBS as u32];
        for (i, dw) in entry.into_iter().enumerate() {
            let _ = erst.write_u32(4 * i, dw);
        }
        Ok(Self {
            segment,
            erst,
            consumer_cycle: 1,
            dequeue_index: 0,
            drained_total: 0,
            armed: [None; ARMED],
            irq: IrqWait::new(0),
            op_base: 0,
        })
    }
}
