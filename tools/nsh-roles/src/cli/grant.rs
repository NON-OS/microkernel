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

//! `grant` and `approve`. A grant id is 16 fresh random bytes.

use super::args::Args;
use super::make::{anchor, number};
use nsh_enrol::files::{now_ms, read, write_new};
use nsh_enrol::{hex, rand};
use nsh_roles_tool::error::{value, Error};
use nsh_roles_tool::grant::GrantSpec;
use nsh_roles_tool::{approve, grant, keys, show};
use nsh_wire::GrantKind;

pub fn grant(a: &Args) -> Result<(), Error> {
    let key = keys::secret(a.one("key")?)?;
    let (table, anchor) = (read(a.one("table")?)?, anchor(a)?);
    let mut grant_id = [0u8; 16];
    if !rand::fill(&mut grant_id) || grant_id == [0; 16] {
        return Err(Error::Enrol(nsh_enrol::Error::Randomness));
    }
    let (kind, parent_id) = match a.maybe("parent")? {
        Some(p) => (GrantKind::Escalation, hex::array(p, "--parent")?),
        None => (GrantKind::Base, [0u8; 16]),
    };
    let now = now_ms()?;
    let ms = number(a, "hours")?.checked_mul(3_600_000).ok_or_else(|| value("--hours"))?;
    if kind == GrantKind::Escalation && ms > nsh_roles::MAX_ESCALATION_MS {
        return Err(value("--hours: an escalation spans at most one hour"));
    }
    let spec = GrantSpec {
        kind,
        grant_id,
        parent_id,
        subject: keys::subject(&keys::public(a.one("subject")?)?[..]),
        user: a.one("user")?.to_string(),
        role: a.one("role")?.to_string(),
        caps: a.all("cap").into_iter().map(str::to_string).collect(),
        not_before_ms: now,
        not_after_ms: now.checked_add(ms).ok_or_else(|| value("--hours"))?,
    };
    let bytes = grant::grant(&table, &anchor, &spec, &key)?;
    write_new(a.one("out")?, &bytes, 0o644)?;
    print!("{}", show::grant(&table, &anchor, &bytes)?);
    Ok(())
}

pub fn approve(a: &Args) -> Result<(), Error> {
    let key = keys::secret(a.one("key")?)?;
    let (table, anchor) = (read(a.one("table")?)?, anchor(a)?);
    let bytes = approve::approve(&table, &anchor, &read(a.one("grant")?)?, &key)?;
    write_new(a.one("out")?, &bytes, 0o644)?;
    print!("{}", show::grant(&table, &anchor, &bytes)?);
    Ok(())
}
