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

//! The bodies of the record's entry kinds. Each is read exactly; a kind no
//! version defines is refused, as is an end reason or a refusal code that
//! is out of range.

use crate::cap_names::MAX_CAP_NAME;
use crate::names;
use crate::record::boxed::BoxRun;
use crate::record::entry::EntryBody;
use crate::{CapNames, Reader, Reason, WireError};

impl<'a> EntryBody<'a> {
    pub fn parse(kind: u8, body: &'a [u8]) -> Result<Self, WireError> {
        let mut r = Reader::new(body);
        let entry = match kind {
            1 => EntryBody::Command(r.take(body.len())?),
            2 => EntryBody::Output { bytes: r.u64()?, digest: r.array()? },
            3 => {
                let tag = r.u32()?;
                let cap = r.bytes8(1, MAX_CAP_NAME)?;
                names::cap(cap)?;
                EntryBody::RefusedCall { tag, cap }
            }
            4 => {
                let (grant, expires_ms) = (r.array()?, r.u64()?);
                let caps = CapNames::read(&mut r)?;
                if caps.is_empty() {
                    return Err(WireError::Value);
                }
                EntryBody::EscGranted { grant, expires_ms, caps }
            }
            5 => match Reason::from_code(r.u16()?)? {
                reason if reason.refuses_escalation() => {
                    EntryBody::EscRefused { reason, grant: r.array()? }
                }
                _ => return Err(WireError::Value),
            },
            6 => {
                let grant = r.array()?;
                match r.u8()? {
                    why @ 1..=3 => EntryBody::EscEnded { grant, why },
                    _ => return Err(WireError::Value),
                }
            }
            7 => EntryBody::Exit(i32::from_le_bytes(*r.array()?)),
            8 => EntryBody::CommandDigest(r.array()?),
            9 => EntryBody::Box(BoxRun::read(&mut r)?),
            _ => return Err(WireError::Value),
        };
        r.finish().map(|_| entry)
    }
}
