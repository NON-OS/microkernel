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
use super::error::BlkError;
use super::read_seq::read_seq;
use super::rearm::rearm;
use super::wait_used::wait_used;
use crate::constants::{VIRTIO_BLK_S_IOERR, VIRTIO_BLK_S_OK, VIRTIO_BLK_S_UNSUPP};
use crate::queue::{Direction, Queue};
use crate::transport::Transport;

pub fn submit(
    transport: Transport,
    queue: &mut Queue,
    irq_grant: u64,
    dir: Direction,
    lba: u64,
    nsectors: u32,
) -> Result<(), BlkError> {
    // The slot is free only once every request posted before has its answer.
    super::settle::settle(transport, queue, irq_grant)?;
    queue.post_request(dir, lba, nsectors);
    /*
     * The sequence is read before the device is told, not after: a device
     * that completes at once raises its interrupt between the two, and a
     * snapshot taken after it waits for a second one that never comes, the
     * whole slice long. Every request paid 100 ms that way.
     */
    let seq = read_seq(irq_grant)?;
    /*
     * Then the line is lowered and unmasked, still before the device is
     * told, or its completion raises no interrupt to wait for (see `rearm`).
     * The snapshot must come first. The line can be shared (on QEMU's q35
     * the display controller sits on it with its status never read), so the
     * unmask can fire at once and the kernel masks the line again. Counted after
     * the snapshot, that interrupt ends the first wait below and the loop
     * unmasks again; counted inside it, as it was when the unmask came
     * first, it left the line masked with the completion still to come, and
     * on several CPUs about every other request slept out its whole slice.
     */
    rearm(transport, irq_grant)?;
    transport.notify(0);
    wait_used(transport, queue, irq_grant, seq)?;
    let status = queue.status_byte();
    rearm(transport, irq_grant)?;
    match status {
        VIRTIO_BLK_S_OK => Ok(()),
        VIRTIO_BLK_S_IOERR => Err(BlkError::Io),
        VIRTIO_BLK_S_UNSUPP => Err(BlkError::Unsupported),
        _ => Err(BlkError::Io),
    }
}
