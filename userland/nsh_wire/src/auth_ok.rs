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

//! `AUTH_OK`: the session id, the idle timeout, and the capabilities the
//! session capsule was actually given.

use crate::{CapNames, Reader, WireError, Writer};

pub const SESSION_ID_LEN: usize = 16;
/// The longest idle timeout a server may state: one day.
pub const MAX_IDLE_S: u32 = 86_400;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AuthOk<'a> {
    pub session_id: &'a [u8; SESSION_ID_LEN],
    pub idle_s: u32,
    pub caps: CapNames<'a>,
}

impl<'a> AuthOk<'a> {
    pub fn parse(body: &'a [u8]) -> Result<Self, WireError> {
        let mut r = Reader::new(body);
        let session_id = r.array()?;
        let idle_s = idle(r.u32()?)?;
        let caps = CapNames::read(&mut r)?;
        r.finish()?;
        Ok(AuthOk { session_id, idle_s, caps })
    }

    pub fn encode(
        session_id: &[u8; SESSION_ID_LEN],
        idle_s: u32,
        caps: &[&[u8]],
        out: &mut [u8],
    ) -> Result<usize, WireError> {
        let mut w = Writer::new(out);
        w.put(session_id)?;
        w.u32(idle(idle_s)?)?;
        CapNames::write(&mut w, caps)?;
        Ok(w.done())
    }
}

fn idle(s: u32) -> Result<u32, WireError> {
    match (1..=MAX_IDLE_S).contains(&s) {
        true => Ok(s),
        false => Err(WireError::Value),
    }
}
