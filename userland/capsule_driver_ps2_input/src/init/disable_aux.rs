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
use super::wait::{wait_input_clear, CTL_TIMEOUT_MS};
use crate::constants::{CTL_DISABLE_AUX, STATUS_OFFSET};
use nonos_libc::mk_pio_write;

pub fn disable_aux(grant_id: u64) {
    if wait_input_clear(grant_id, CTL_TIMEOUT_MS).is_ok() {
        let _ = mk_pio_write(grant_id, STATUS_OFFSET, 1, CTL_DISABLE_AUX as u32);
    }
}
