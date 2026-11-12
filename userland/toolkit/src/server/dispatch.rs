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

use crate::animation;
use crate::protocol::{E_BAD_OP, STATUS_OK, TOOLKIT_OP_ANIMATION_TICK, TOOLKIT_OP_HEALTHCHECK};

pub fn dispatch(op: u16, payload: &[u8], reply: &mut [u8]) -> (u16, usize) {
    match op {
        TOOLKIT_OP_HEALTHCHECK => (STATUS_OK, 0),
        TOOLKIT_OP_ANIMATION_TICK => animation::tick(payload, reply),
        _ => (E_BAD_OP, 0),
    }
}
