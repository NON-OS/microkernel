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

//! The rules a grant's bytes must keep: its kind, ids that are not zero,
//! and a parent only an escalation has, never itself.

use crate::grant::types::{GrantKind, GRANT_ID_LEN, GRANT_MAGIC, GRANT_VERSION};
use crate::{Reader, WireError};

pub(crate) fn head(r: &mut Reader<'_>) -> Result<GrantKind, WireError> {
    if r.array::<4>()? != &GRANT_MAGIC {
        return Err(WireError::Magic);
    }
    if r.u16()? != GRANT_VERSION {
        return Err(WireError::BodyVersion);
    }
    let kind = match r.u8()? {
        1 => GrantKind::Base,
        2 => GrantKind::Escalation,
        _ => return Err(WireError::Value),
    };
    r.zero(1).map(|_| kind)
}

pub(crate) fn nonzero<const N: usize>(id: &[u8; N]) -> Result<&[u8; N], WireError> {
    id.iter().any(|b| *b != 0).then_some(id).ok_or(WireError::Value)
}

type Id<'a> = &'a [u8; GRANT_ID_LEN];

pub(crate) fn parent<'a>(kind: GrantKind, id: Id<'a>, p: Id<'a>) -> Result<Id<'a>, WireError> {
    let ok = match kind {
        GrantKind::Base => p.iter().all(|b| *b == 0),
        GrantKind::Escalation => nonzero(p).is_ok() && p != id,
    };
    ok.then_some(p).ok_or(WireError::Value)
}

type Shape = fn(&[u8]) -> Result<(), WireError>;

pub(crate) fn named(name: &[u8], shape: Shape) -> Result<&[u8], WireError> {
    shape(name).map(|_| name)
}
