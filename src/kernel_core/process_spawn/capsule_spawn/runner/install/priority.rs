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

use crate::process::core::Priority;

/// Capsules on the pointer/keyboard -> compositor -> scanout path.
///
/// Each one parks in the kernel when it has nothing to do (`mk_irq_wait`,
/// `mk_input_event_wait`, `mk_ipc_recv_from` with a timeout), so the band is
/// empty whenever the desktop is idle. `select_by_priority` also skips the
/// running process, so a band holding a single runnable member always falls
/// through to `Normal` on the next switch - a promoted capsule that spun
/// would cost half the CPU, never all of it.
const INTERACTIVE: [&str; 4] =
    ["driver.ps2_kbd0", "input_router", "compositor", "driver.virtio_gpu0"];

/*
 * Capsules that move a frame between the card and TCP: the card drivers and
 * the stack, which is where acknowledgements are made. They park the same way
 * (`mk_irq_wait`, `mk_ipc_recv_from` with a timeout), so the band stays empty
 * on an idle network. In the Normal band a wake queued behind every runnable
 * capsule, and a capture of a 92 KB page load showed a median of 230 ms
 * between the guest's acknowledgements, which set the pace of every
 * slow-start round.
 *
 * `net.sockets` and `net.tcp` stay Normal: they wait for a connection by
 * yielding in a loop, and two of those in this band could hold it for a whole
 * connect timeout.
 */
const PACKET_PATH: [&str; 7] = [
    "driver.virtio_net0",
    "driver.e1000_0",
    "driver.rtl8169_0",
    "driver.rtl8139_0",
    "driver.iwlwifi0",
    "driver.rtl8821ce0",
    "net.core",
];

/*
 * The disk drivers. Each waits in `mk_ipc_recv_from` with no timeout and
 * sleeps on its interrupt while a request is out, so the band stays empty on
 * an idle disk. In the Normal band, a Linux guest with every CPU busy (a model
 * answering on four threads) kept a driver off the CPU past its request's
 * budget, and a model load died on a read that a disk had long finished.
 */
const STORAGE_PATH: [&str; 4] =
    ["driver.virtio_blk0", "driver.ahci0", "driver.nvme0", "driver.usb_msc0"];

/// Scheduling band a freshly installed capsule starts in.
pub(super) fn for_capsule(name: &str) -> Priority {
    if INTERACTIVE.contains(&name) || PACKET_PATH.contains(&name) || STORAGE_PATH.contains(&name) {
        Priority::High
    } else {
        Priority::Normal
    }
}
