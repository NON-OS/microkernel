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

use super::serve::serve;
use super::walk::walk;
use crate::discover::Found;
use crate::error::{AhciError, AhciResult};
use crate::setup::Driver;

/// Walk every AHCI controller discovery found and every implemented SATA
/// port on each, then serve the one disk that carries NONOS (or the
/// blank-target fallback). Fails when no disk came up, and then every
/// controller the walk touched has been released again (`open` rolls back a
/// controller it could not finish), so the next attempt can claim them.
pub fn run(found: &[Found]) -> AhciResult<Driver> {
    if found.is_empty() {
        return Err(AhciError::DeviceNotFound);
    }
    serve(walk(found))
}
