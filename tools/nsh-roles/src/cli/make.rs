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

//! `table` and `revoke`.

use super::args::Args;
use nsh_enrol::files::{now_ms, read, write_new};
use nsh_enrol::hex;
use nsh_roles_tool::error::{value, Error};
use nsh_roles_tool::table::TableSpec;
use nsh_roles_tool::{keys, revoke, roles_file, table};

pub(super) fn number(a: &Args, name: &str) -> Result<u64, Error> {
    a.one(name)?.parse().map_err(|_| value(&format!("--{name}")))
}

pub(super) fn anchor(a: &Args) -> Result<[u8; 32], Error> {
    Ok(hex::array(a.one("anchor")?, "--anchor")?)
}

pub fn table(a: &Args) -> Result<(), Error> {
    let key = keys::secret(a.one("key")?)?;
    let publics = |n: &str| a.all(n).into_iter().map(keys::public).collect::<Result<Vec<_>, _>>();
    let roles = match a.maybe("roles")? {
        Some(f) => roles_file::parse(&String::from_utf8(read(f)?).map_err(|_| value("--roles"))?)?,
        None => nsh_roles_tool::shipped::shipped(),
    };
    let hours = number(a, "max-age-hours")?;
    let spec = TableSpec {
        fleet: hex::array(a.one("fleet")?, "--fleet")?,
        serial: number(a, "serial")?,
        issued_ms: now_ms()?,
        max_age_ms: hours.checked_mul(3_600_000).ok_or_else(|| value("--max-age-hours"))?,
        authorities: publics("authority")?,
        approvers: publics("approver")?,
        roles,
    };
    let bytes = table::table(spec, &key)?;
    write_new(a.one("out")?, &bytes, 0o644)?;
    println!("table anchor {}", hex::of(&key.id()));
    Ok(())
}

pub fn revoke(a: &Args) -> Result<(), Error> {
    let key = keys::secret(a.one("key")?)?;
    let table = read(a.one("table")?)?;
    let ids =
        a.all("id").into_iter().map(|h| hex::array::<16>(h, "--id")).collect::<Result<_, _>>()?;
    let bytes = revoke::revoke(&table, &anchor(a)?, now_ms()?, ids, &key)?;
    write_new(a.one("out")?, &bytes, 0o644)?;
    print!("{}", nsh_roles_tool::show::list(&table, &anchor(a)?, &bytes)?);
    Ok(())
}
