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

//! The capability bits, the cards, and a caller owning given endpoints.

pub(super) const IPC: u64 = 1 << 3;
pub(super) const NETWORK: u64 = 1 << 2;

pub(super) const WIRED: [&str; 4] =
    ["driver.virtio_net0", "driver.e1000_0", "driver.rtl8169_0", "driver.rtl8139_0"];
pub(super) const WIFI: [&str; 2] = ["driver.iwlwifi0", "driver.rtl8821ce0"];

/// A caller that owns exactly the endpoints `names`.
pub(super) fn owner(names: &'static [&'static str]) -> impl Fn(&str) -> bool {
    move |n| names.contains(&n)
}
