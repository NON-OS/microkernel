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

//! Taking the host for a transfer (ICH and PCH datasheets, Host Status).

use crate::errno::E_BUSY;
use crate::ports::Ports;
use crate::regs::{HSTSTS, STS_FLAGS, STS_HOST_BUSY};

/// A running host is not ours to start. An end a previous transaction left
/// recorded is cleared, and one that will not clear means busy too.
pub fn take<P: Ports>(p: &mut P) -> Result<(), i32> {
    let st = p.r8(HSTSTS)?;
    if st & STS_HOST_BUSY != 0 {
        return Err(E_BUSY);
    }
    if st & STS_FLAGS != 0 {
        p.w8(HSTSTS, st & STS_FLAGS)?;
        if p.r8(HSTSTS)? & STS_FLAGS != 0 {
            return Err(E_BUSY);
        }
    }
    Ok(())
}
