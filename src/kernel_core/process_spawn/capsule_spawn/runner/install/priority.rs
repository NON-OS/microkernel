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

use super::priority_vm::{INTERACTIVE_VM, STORAGE_PATH_VM};
use crate::process::core::Priority;
use crate::services::registry::displays::is_display;
use crate::services::registry::nics;

/// Capsules on the pointer/keyboard -> compositor -> scanout path; the
/// display drivers that scan out are every name in `displays.rs`.
///
/// Each one parks in the kernel when it has nothing to do (`mk_irq_wait`,
/// `mk_input_event_wait`, `mk_ipc_recv_from` with a timeout), so the band is
/// empty whenever the desktop is idle. `select_by_priority` also skips the
/// running process, so a band holding a single runnable member always falls
/// through to `Normal` on the next switch - a promoted capsule that spun
/// would cost half the CPU, never all of it.
const INTERACTIVE: [&str; 3] = ["driver.ps2_kbd0", "input_router", "compositor"];

/*
 * Capsules that move a frame between the card and TCP: the card drivers
 * (registry/nics.rs) and the stack, which is where acknowledgements are
 * made. They park the same way (`mk_irq_wait`, `mk_ipc_recv_from` with a
 * timeout), so the band stays empty on an idle network. In the Normal band a
 * wake queued behind every runnable capsule, and a capture of a 92 KB page
 * load showed a median of 230 ms between the guest's acknowledgements, which
 * set the pace of every slow-start round.
 *
 * `net.sockets` and `net.tcp` stay Normal: they wait for a connection by
 * yielding in a loop, and two of those in this band could hold it for a whole
 * connect timeout.
 */
const PACKET_PATH: [&str; 1] = ["net.core"];

/*
 * The disk drivers. Each waits in `mk_ipc_recv_from` with no timeout and
 * sleeps on its interrupt while a request is out, so the band stays empty on
 * an idle disk. In the Normal band, a Linux guest with every CPU busy (a model
 * answering on four threads) kept a driver off the CPU past its request's
 * budget, and a model load died on a read that a disk had long finished.
 */
const STORAGE_PATH: [&str; 6] = [
    "driver.virtio_blk0",
    "driver.ahci0",
    "driver.nvme0",
    "driver.usb_msc0",
    "driver.sdhci0",
    "driver.ufs0",
];

/// Scheduling band a freshly installed capsule starts in.
pub(super) fn for_capsule(name: &str) -> Priority {
    let bands: [&[&str]; 5] =
        [&INTERACTIVE, &INTERACTIVE_VM, &PACKET_PATH, &STORAGE_PATH, &STORAGE_PATH_VM];
    let banded = bands.iter().any(|band| band.contains(&name));
    if is_display(name) || banded || nics::NICS.iter().any(|(card, _)| *card == name) {
        Priority::High
    } else {
        Priority::Normal
    }
}
