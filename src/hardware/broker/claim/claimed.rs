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

//! Whether a device has a claim at all, at any stage: being made, held, or
//! being given back. For a rule that must not act under another holder's
//! feet (pci/companion.rs), where a claim on its way in or out still counts.

use super::state::claims;

pub(crate) fn claimed(device_id: u64) -> bool {
    claims().iter().any(|c| c.device_id == device_id)
}
