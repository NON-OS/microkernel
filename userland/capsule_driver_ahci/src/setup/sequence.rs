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
use crate::discover::find_ahci;
use crate::error::{AhciError, AhciResult};
use crate::setup::Driver;

/// Walk every AHCI controller and every present SATA port on each, then serve
/// the one disk that carries NONOS (or the blank-target fallback). Fails only
/// when no controller could be opened.
pub fn run() -> AhciResult<Driver> {
    let found = find_ahci();
    if found.is_empty() {
        return Err(AhciError::DeviceNotFound);
    }
    let w = walk(&found);
    let first_error = w.first_error;
    serve(w).ok_or(first_error.unwrap_or(AhciError::DeviceNotFound))
}
