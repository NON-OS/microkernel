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

//! A board's records put on this thread's device list as the kernel shows
//! them, and what the shipping discovery reads back from it. Pads register
//! after the controllers, each taking the highest id so far plus one. With
//! the host-by-id kernel, a caller that does not hold a device reads 0 for
//! each bar base but a kind-None bar's; the kernel before it showed all.

use nonos_libc::{list_devices, DeviceRecord, BAR_KIND_NONE};

use crate::discover::{find_controllers, find_touchpad_addrs, AcpiTouchpad, Found};
use crate::discover::{MAX_CONTROLLERS, MAX_TARGETS};
use crate::setup::order::probe_order;

/// The table: `controllers`, then `pads` registered after them.
pub fn table(controllers: &[DeviceRecord], pads: &[DeviceRecord]) -> Vec<DeviceRecord> {
    let top = controllers.iter().map(|r| r.device_id).max().unwrap_or(0);
    let ids = (1..).map(|k| top.wrapping_add(k));
    let pads = pads.iter().zip(ids).map(|(p, device_id)| DeviceRecord { device_id, ..*p });
    controllers.iter().copied().chain(pads).collect()
}

/// `r` as a caller holding `held` sees it.
fn shown(r: DeviceRecord, held: Option<u64>) -> DeviceRecord {
    let mut seen = r;
    if held != Some(r.device_id) {
        seen.bars.iter_mut().filter(|b| b.kind != BAR_KIND_NONE).for_each(|b| b.base = 0);
    }
    seen
}

/// `records` as a caller holding `held` sees them.
pub fn seen(records: &[DeviceRecord], held: Option<u64>) -> Vec<DeviceRecord> {
    records.iter().map(|r| shown(*r, held)).collect()
}

/// List `records` as a caller holding `held` sees them.
pub fn list(records: &[DeviceRecord], held: Option<u64>) {
    list_devices(Ok(seen(records, held)));
}

/// The controllers and pads the shipping discovery reads from the list.
pub fn discovered() -> (Vec<Found>, Vec<AcpiTouchpad>) {
    let mut found = [Found::default(); MAX_CONTROLLERS];
    let mut pads = [AcpiTouchpad::default(); MAX_TARGETS];
    let (n, t) = (find_controllers(&mut found), find_touchpad_addrs(&mut pads));
    (found[..n].to_vec(), pads[..t].to_vec())
}

/// The order probe_pass.rs tries `cands` in.
pub fn order(cands: &[Found], targets: &[AcpiTouchpad]) -> Vec<usize> {
    let (order, n) = probe_order(cands, targets);
    order[..n].to_vec()
}
