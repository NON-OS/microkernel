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

//! Writing a list of capability names. The names are put in order, the list
//! is built in a scratch copy and read back with the same checks a receiver
//! makes, so an encoder can never send a list its peer would refuse.

use crate::cap_names::{MAX_CAPS, MAX_CAP_NAME};
use crate::{CapNames, Reader, WireError, Writer};

impl<'a> CapNames<'a> {
    pub fn write(w: &mut Writer<'_>, names: &[&[u8]]) -> Result<(), WireError> {
        if names.len() > MAX_CAPS {
            return Err(WireError::Oversize);
        }
        let mut sorted: [&[u8]; MAX_CAPS] = [&[]; MAX_CAPS];
        sorted[..names.len()].copy_from_slice(names);
        let sorted = &mut sorted[..names.len()];
        sorted.sort_unstable();
        let count = names.len() as u8;
        let mut copy = [0u8; MAX_CAPS * (1 + MAX_CAP_NAME)];
        let mut cw = Writer::new(&mut copy);
        cw.u8(count)?;
        sorted.iter().try_for_each(|n| cw.bytes8(n))?;
        let done = cw.done();
        CapNames::read(&mut Reader::new(&copy[..done]))?;
        w.put(&copy[..done])
    }
}
