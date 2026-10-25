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

//! A device's port release keyed by its claim (L71), where there are no
//! ports. `pio_absent` stays as its extraction mirrors it; this only takes
//! the claim's epoch the other releases take, so `give_back` is one path on
//! every target.

/// Drop every port grant `pid` holds over `device_id` under the claim at
/// `epoch`. None can, so none did.
pub fn pio_release_for_device(pid: u32, device_id: u64, epoch: u64) -> usize {
    let _ = epoch;
    super::pio_release_for_device_unkeyed(pid, device_id)
}
