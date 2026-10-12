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
//! The compositor told whenever the camera starts or stops streaming, so
//! the indicator shows it (indicator.rs). Whether it answered that the
//! indicator is shown is OP_START's to act on.

use nonos_libc::{mk_ipc_call, mk_service_lookup};

use crate::indicator::{notice, shown};

const COMPOSITOR: &str = "compositor";

/// Whether the compositor shows the indicator now; false when there is no
/// compositor or it does not know the op yet.
pub fn capture(on: bool) -> bool {
    let (mut port, mut pid) = (0u32, 0u32);
    if mk_service_lookup(COMPOSITOR.as_ptr(), COMPOSITOR.len(), &mut port, &mut pid) != 0 {
        return false;
    }
    let req = notice(on);
    let mut resp = [0u8; 32];
    let rc = mk_ipc_call(port as u64, req.as_ptr(), req.len(), resp.as_mut_ptr(), resp.len());
    shown(rc, &resp)
}
