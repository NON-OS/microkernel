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

//! The startup-locality event (TCG PC Client Platform Firmware Profile,
//! 10.4.5.3): an `EV_NO_ACTION` in PCR 0 whose body names the locality the
//! TPM was started from. PCR 0 then starts at that value, not at zero. It
//! is honoured once, before anything is extended into PCR 0, and only for
//! the localities a PC starts from; anything else is refused.

use super::Event;
use crate::LogFault;

const SIGNATURE: &[u8; 16] = b"StartupLocality\0";

/// The locality an event sets PCR 0 from, or `None` if it is not one.
pub(super) fn startup_locality(e: &Event<'_>, pcr0_touched: bool) -> Result<Option<u8>, LogFault> {
    if !e.body.starts_with(SIGNATURE) {
        return Ok(None);
    }
    match (e.pcr, e.body.len(), e.body.get(16), pcr0_touched) {
        (0, 17, Some(&l @ (0 | 3 | 4)), false) => Ok(Some(l)),
        _ => Err(LogFault::Locality),
    }
}
