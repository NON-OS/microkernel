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

//! Signing a body, and reading a certificate back.

use nsh_enrol::keys::wipe;
use nsh_enrol::keys::PUBLIC_FILE_LEN;
use nsh_enrol::{hex, AuthorityKey, Error};

use super::args::Args;
use nsh_enrol::files::{now_ms, read, read_exact, write_new};

pub fn sign(a: &Args) -> Result<(), Error> {
    let body = read(a.one("body")?)?;
    if nonos_hash::sha256(&body) != hex::array::<32>(a.one("expect")?, "expect")? {
        return Err(Error::Value("body: not the one `body` made (--expect)"));
    }
    let mut raw = read(a.one("key")?)?;
    let key = AuthorityKey::from_file(&raw);
    wipe(&mut raw);
    let key = key?;
    let cert = nsh_enrol::sign(&body, &key, now_ms()?)?;
    let public: [u8; PUBLIC_FILE_LEN] = key.public().try_into().map_err(|_| Error::KeyFile)?;
    print!("{}", nsh_enrol::describe(&nsh_enrol::verify(&cert, &public)?));
    write_new(a.one("out")?, &cert, 0o644)
}

pub fn show(a: &Args) -> Result<(), Error> {
    let cert = read(a.one("cert")?)?;
    let authority = read_exact::<PUBLIC_FILE_LEN>(a.one("authority")?)?;
    print!("{}", nsh_enrol::describe(&nsh_enrol::verify(&cert, &authority)?));
    Ok(())
}
