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

//! The one-field bodies: a `SIGNAL`, the reason code of an `ABORT` or
//! `AUTH_REFUSED`, and the status of an `EXIT`. Each reason must be one its
//! frame may carry (`reason_sets`); none of them may carry `OK`.

use crate::{Reader, Reason, WireError};

fn reason_in(body: &[u8], set: fn(Reason) -> bool) -> Result<Reason, WireError> {
    let mut r = Reader::new(body);
    let reason = Reason::from_code(r.u16()?)?;
    r.finish()?;
    set(reason).then_some(reason).ok_or(WireError::Value)
}

/// A plaintext `ABORT`, before the keys: `VERSION` or `BUSY` only.
pub fn plain_abort(body: &[u8]) -> Result<Reason, WireError> {
    reason_in(body, Reason::before_keys)
}

/// A sealed `ABORT`.
pub fn abort(body: &[u8]) -> Result<Reason, WireError> {
    reason_in(body, Reason::ends_session)
}

pub fn auth_refused(body: &[u8]) -> Result<Reason, WireError> {
    reason_in(body, Reason::refuses_login)
}

pub fn exit_status(body: &[u8]) -> Result<i32, WireError> {
    let mut r = Reader::new(body);
    let status = i32::from_le_bytes(*r.array()?);
    r.finish().map(|_| status)
}

/// A `CLOSE` has no body.
pub fn empty(body: &[u8]) -> Result<(), WireError> {
    Reader::new(body).finish()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Signal {
    Interrupt = 1,
    End = 2,
}

pub fn signal(body: &[u8]) -> Result<Signal, WireError> {
    match body {
        [1] => Ok(Signal::Interrupt),
        [2] => Ok(Signal::End),
        [_] => Err(WireError::Value),
        [] => Err(WireError::Short),
        _ => Err(WireError::Trailing),
    }
}
