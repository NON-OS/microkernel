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

//! What a translated device must pass before it joins a domain.

use super::refuse::Refused;
use super::table::say;

pub(super) fn admit(device_id: u64) -> Result<(), Refused> {
    super::interrupts::admit()?;
    // A virtio device without ACCESS_PLATFORM ignores the domain (K5).
    if !super::virtio::may_confine(device_id) {
        say(b"refused: virtio without ACCESS_PLATFORM under an IOMMU");
        return Err(Refused);
    }
    Ok(())
}
