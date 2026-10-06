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

//! Who may make the device proof's calls. The DeviceSecret bit alone comes
//! from a signed manifest, bounded only by its publisher's ceiling, so any
//! publisher whose ceiling covers it could ship a capsule that reads the secret
//! and drives the EK and AK, and the proof would name the device. So the bit
//! counts only for a capsule the vendor root proved: the one authority that
//! enrolls nonos.prove. A developer root, a third-party publisher and software
//! built on the machine are refused.

use crate::security::attest_registry::authority_of;
use crate::security::dev_roots::Authority;

pub(super) fn device_secret_caller() -> bool {
    if !crate::syscall::caps::current_caps_or_default().can_device_secret() {
        return false;
    }
    let Some(pid) = crate::process::current_pid() else {
        return false;
    };
    matches!(authority_of(pid), Some(Authority::Vendor))
}
