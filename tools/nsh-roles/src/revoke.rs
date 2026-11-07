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

//! `nsh-revoke/1`, the revocation list: the grant ids a fleet withdraws,
//! signed by a role authority of the table, and opened under that table
//! before it is handed back.

use crate::error::{value, Error};
use crate::put;
use nsh_crypto::{labelled, Label};
use nsh_enrol::AuthorityKey;
use nsh_roles::{open_policy, open_revocations, REVOKE_MAGIC};

/// `table` is the installed table and `anchor` the key id hosts pin it by.
pub fn revoke(
    table: &[u8],
    anchor: &[u8; 32],
    issued_ms: u64,
    mut ids: Vec<[u8; 16]>,
    signer: &AuthorityKey,
) -> Result<Vec<u8>, Error> {
    let policy = open_policy(table, anchor, 0)?;
    ids.sort_unstable();
    ids.dedup();
    let mut out = REVOKE_MAGIC.to_vec();
    put::u16(&mut out, 1);
    put::u16(&mut out, 0);
    out.extend_from_slice(policy.fleet_id);
    put::u64(&mut out, issued_ms);
    put::u16(&mut out, u16::try_from(ids.len()).map_err(|_| value("revoked id count"))?);
    ids.iter().for_each(|id| out.extend_from_slice(id));
    out.extend_from_slice(&signer.id());
    let sig = signer.sign(&labelled(Label::Revoke, &out))?;
    out.extend_from_slice(&sig);
    open_revocations(&out, &policy, 0)?;
    Ok(out)
}
