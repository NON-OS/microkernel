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

//! `show`: a table, or a list or a grant under it.

use super::args::Args;
use super::make::anchor;
use nsh_enrol::files::read;
use nsh_roles_tool::error::Error;
use nsh_roles_tool::show;

pub fn run(a: &Args) -> Result<(), Error> {
    let (table, anchor) = (read(a.one("table")?)?, anchor(a)?);
    let text = match (a.maybe("list")?, a.maybe("grant")?) {
        (Some(l), None) => show::list(&table, &anchor, &read(l)?)?,
        (None, Some(g)) => show::grant(&table, &anchor, &read(g)?)?,
        (None, None) => show::table(&table, &anchor)?,
        (Some(_), Some(_)) => return Err(Error::Usage("--list or --grant, not both".into())),
    };
    print!("{text}");
    Ok(())
}
