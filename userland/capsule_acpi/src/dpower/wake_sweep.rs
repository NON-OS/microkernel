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

//! Which armed devices lose their wake: each stays armed only while the
//! kernel still says its owner holds it, by the same claim test
//! DEVICE_POWER makes. A claim released, an owner that exited, and a
//! device that departed all fail that test, so each disarms it. Pure.

use alloc::vec::Vec;

/// An armed device's namespace id, its owner's pid and the device id the
/// owner named it by.
pub type Owner = (u32, u32, u64);

/// The owners whose device `holds(pid, device id)` no longer resolves to
/// for them, newest armed first (`owners` is in arming order), so they
/// are disarmed in reverse; None from `holds` is a refusal of any kind.
pub fn stale(owners: &[Owner], holds: &mut dyn FnMut(u32, u64) -> Option<u32>) -> Vec<Owner> {
    owners.iter().rev().copied().filter(|o| holds(o.1, o.2) != Some(o.0)).collect()
}

/// The owners of devices that departed, by device id, newest first.
pub fn departed(owners: &[Owner], ids: &[u64]) -> Vec<Owner> {
    owners.iter().rev().copied().filter(|o| ids.contains(&o.2)).collect()
}

/// Whether `sender` armed `dev` and so may read its wake count.
pub fn owns(owners: &[Owner], dev: u32, sender: u32) -> bool {
    owners.iter().any(|o| o.0 == dev && o.1 == sender)
}
