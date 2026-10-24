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

//! `nsh-verify-record`: checks a session record offline (nsh-v1-record.md).
//! This build holds no `NONOSATT` reader, so step 2 refuses every
//! document as `Unavailable` until one is linked in its place.

use nsh_client::{AttestRefusal, Attester, Quoted};

struct NoReader;

impl Attester for NoReader {
    fn check(&self, _: &[u8], _: &[u8; 32], _: Option<&[u8]>) -> Result<Quoted, AttestRefusal> {
        Err(AttestRefusal::Unavailable)
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let (code, line) = nsh_client::cli::run(&args, &NoReader);
    match code {
        0 | 3 => println!("{line}"),
        _ => eprintln!("{line}"),
    }
    std::process::exit(code);
}
