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

//! How the ACPI holder calls name who is asking: a mode word, 0 for the
//! capsule's own look (the pid must then be 0) and 1 for a driver's
//! request (the pid must then be that driver's), so no pid value means
//! anything by itself.

use crate::hardware::broker::acpi::claim_holder::Asker;

const LOOK: u64 = 0;
const HOLDER: u64 = 1;

pub(super) fn asker(mode: u64, pid: u64) -> Option<Asker> {
    match (mode, u32::try_from(pid).ok()?) {
        (LOOK, 0) => Some(Asker::Anyone),
        (HOLDER, pid) if pid != 0 => Some(Asker::Holder(pid)),
        _ => None,
    }
}
