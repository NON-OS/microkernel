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

//! The certificate body.

use nsh_enrol::keys::PUBLIC_FILE_LEN;
use nsh_enrol::{hex, Error, Request};

use super::args::Args;
use nsh_enrol::files::{now_ms, read, read_exact, write_new};

const DAY_MS: u64 = 86_400_000;

pub fn body(a: &Args) -> Result<(), Error> {
    let doc = read(a.one("doc")?)?;
    let host_keys = read_exact::<PUBLIC_FILE_LEN>(a.one("host-keys")?)?;
    let authority = read_exact::<PUBLIC_FILE_LEN>(a.one("authority")?)?;
    let digests =
        |name, what| a.all(name).iter().map(|l| hex::array(l, what)).collect::<Result<Vec<_>, _>>();
    let (loaders, boot_apps) =
        (digests("loader", "loader digest")?, digests("boot-app", "boot app")?);
    let days: u64 = a.one("days")?.parse().map_err(|_| Error::Value("days"))?;
    if !(1..=3650).contains(&days) {
        return Err(Error::Value("days: 1 to 3650"));
    }
    let nonce = hex::array(a.one("nonce")?, "nonce")?;
    let now_ms = now_ms()?;
    let req = Request {
        doc: &doc,
        challenge: nsh_enrol::challenge(&nonce, &host_keys),
        ak: hex::array(a.one("ak")?, "ak")?,
        host_keys: &host_keys,
        host: a.one("host")?.as_bytes(),
        fleet: hex::array(a.one("fleet")?, "fleet id")?,
        loaders: &loaders,
        boot_apps: &boot_apps,
        not_after_ms: now_ms + days * DAY_MS,
        now_ms,
        authority_id: nonos_hash::sha256(&authority[..]),
    };
    let body = nsh_enrol::body(&req)?;
    write_new(a.one("out")?, &body, 0o644)?;
    print!("{}", nsh_enrol::describe_body(&body)?);
    println!("body sha256 {}  (give it to sign as --expect)", hex::of(&nonos_hash::sha256(&body)));
    Ok(())
}
