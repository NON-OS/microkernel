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
//! - `keygen --out DIR --name NAME`: a new hybrid key, `NAME.secret`
//!   (mode 0600) and `NAME.public`, and its key id.
//! - `table --fleet HEX --serial N --max-age-hours N --authority F ...
//!   [--approver F ...] [--roles F] --key F --out F`: the role table, signed
//!   by `--key`, which must be one of the authorities; its key id is the
//!   anchor hosts pin.
//! - `revoke --table F --anchor HEX [--id HEX ...] --key F --out F`: a
//!   revocation list issued now, signed by a role authority.
//! - `grant --table F --anchor HEX --subject F --user NAME --role NAME
//!   --cap NAME ... --hours N [--parent HEX] --key F --out F`: a base grant,
//!   or with `--parent` an escalation of it, from now for `--hours`.
//! - `approve --table F --anchor HEX --grant F --key F --out F`: the grant
//!   with this approver's approval added.
//! - `show --table F --anchor HEX [--list F | --grant F]`: checked, and
//!   shown field by field.
//!
//! No command overwrites a file.

mod args;
mod grant;
mod keygen;
mod make;
mod show;

use nsh_roles_tool::Error;

const USAGE: &str =
    "usage: nsh-roles keygen | table | revoke | grant | approve | show (see the README)";

pub fn run(argv: &[String]) -> Result<(), Error> {
    let (cmd, rest) = argv.split_first().ok_or(Error::Usage(USAGE.into()))?;
    let a = args::Args::parse(rest)?;
    match cmd.as_str() {
        "keygen" => keygen::run(&a),
        "table" => make::table(&a),
        "revoke" => make::revoke(&a),
        "grant" => grant::grant(&a),
        "approve" => grant::approve(&a),
        "show" => show::run(&a),
        _ => Err(Error::Usage(USAGE.into())),
    }
}
