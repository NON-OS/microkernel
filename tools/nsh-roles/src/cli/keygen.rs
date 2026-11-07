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

//! A new hybrid key: a role authority's, an approver's or a user's.

use super::args::Args;
use nsh_enrol::files::write_new;
use nsh_enrol::keys::wipe;
use nsh_enrol::{hex, AuthorityKey};
use nsh_roles_tool::error::{value, Error};

pub fn run(a: &Args) -> Result<(), Error> {
    let (dir, name) = (a.one("out")?, a.one("name")?);
    let ok = |c: char| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '_';
    if name.is_empty() || !name.chars().all(ok) {
        return Err(value("--name: lower-case letters, digits, '-' and '_'"));
    }
    let key = AuthorityKey::generate()?;
    let (secret, public) = (format!("{dir}/{name}.secret"), format!("{dir}/{name}.public"));
    let mut file = key.to_file();
    let written = write_new(&secret, &file, 0o600);
    wipe(&mut file);
    written?;
    write_new(&public, &key.public(), 0o644)?;
    println!("{name} key id {}", hex::of(&key.id()));
    Ok(())
}
