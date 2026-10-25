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

//! A new fleet authority key.

use nsh_enrol::keys::wipe;
use nsh_enrol::{hex, AuthorityKey, Error};

use super::args::Args;
use nsh_enrol::files::write_new;

pub fn run(a: &Args) -> Result<(), Error> {
    let dir = a.one("out")?;
    let key = AuthorityKey::generate()?;
    let (secret, public) = (format!("{dir}/authority.secret"), format!("{dir}/authority.public"));
    if [&secret, &public].iter().any(|p| std::path::Path::new(p).exists()) {
        return Err(Error::Io(format!("{dir}: a key is already there")));
    }
    let mut file = key.to_file();
    let written = write_new(&secret, &file, 0o600);
    wipe(&mut file);
    written?;
    write_new(&public, &key.public(), 0o644)?;
    println!("authority key id {}", hex::of(&key.id()));
    Ok(())
}
