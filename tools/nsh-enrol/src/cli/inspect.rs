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

//! `inspect --doc F | --serial F [--challenge HEX]`: a machine's document,
//! checked and described. `--serial` reads the one a development image
//! prints after its boot, between `[ATTEST-DOC] begin N` and `end`.

use nsh_enrol::{hex, Error};

use super::args::Args;
use nsh_enrol::files::read;

pub fn run(a: &Args) -> Result<(), Error> {
    let doc = match (a.maybe("doc")?, a.maybe("serial")?) {
        (Some(f), None) => read(f)?,
        (None, Some(f)) => nsh_enrol::doc_from_serial(&String::from_utf8_lossy(&read(f)?))?,
        _ => return Err(Error::Usage("give one of --doc or --serial".into())),
    };
    let challenge = a.maybe("challenge")?.map(|c| hex::array(c, "challenge")).transpose()?;
    print!("{}", nsh_enrol::inspect(&doc, challenge)?);
    Ok(())
}
