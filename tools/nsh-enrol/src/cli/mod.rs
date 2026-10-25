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

//! The commands:
//!
//! - `keygen --out DIR`: a new fleet authority key, `authority.secret`
//!   (mode 0600) and `authority.public`, and its key id.
//! - `challenge --host-keys F`: a fresh nonce, and the challenge the
//!   machine's document must answer for those keys.
//! - `body --doc F --nonce HEX --ak HEX --host-keys F --host NAME
//!   --fleet HEX --loader HEX [--loader HEX ...] [--boot-app HEX ...]
//!   --days N --authority F --out F`: the unsigned certificate body, shown,
//!   and its SHA-256.
//! - `sign --body F --expect HEX --key F --out F`: the signed certificate,
//!   checked and shown before it is written.
//! - `show --cert F --authority F`: a checked certificate, field by field.
//! - `inspect --doc F | --serial F [--challenge HEX]`: a machine's document,
//!   checked as far as it can be without an enrolment, and described.
//!
//! No command overwrites a file, so a key or a certificate is never lost
//! to a mistyped path.

mod args;
mod challenge;
mod inspect;
mod keygen;
mod make;
mod sign;

use nsh_enrol::Error;

const USAGE: &str =
    "usage: nsh-enrol keygen | challenge | body | sign | show | inspect (see the README)";

pub fn run(argv: &[String]) -> Result<(), Error> {
    let (cmd, rest) = argv.split_first().ok_or(Error::Usage(USAGE.into()))?;
    let a = args::Args::parse(rest)?;
    match cmd.as_str() {
        "keygen" => keygen::run(&a),
        "challenge" => challenge::run(&a),
        "body" => make::body(&a),
        "sign" => sign::sign(&a),
        "show" => sign::show(&a),
        "inspect" => inspect::run(&a),
        _ => Err(Error::Usage(USAGE.into())),
    }
}
