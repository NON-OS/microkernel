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

//! Whether a stream socket may listen. A listener takes connections from
//! whoever can reach the machine, outside any mixnet, so it is opened only
//! while the person's chosen network is Direct. Under Nym or Anyone, or with
//! a policy store that does not answer, it is refused as having no
//! transport, the answer the desktop's net.core already gives. Pure, so the
//! host proofs hold it.

use crate::protocol::E_NO_TRANSPORT;

/// Ok when a listener may be opened; `direct` says whether the chosen
/// network is Direct, asked for each listen.
pub fn listen_route(direct: impl FnOnce() -> bool) -> Result<(), u16> {
    if direct() {
        Ok(())
    } else {
        Err(E_NO_TRANSPORT)
    }
}
