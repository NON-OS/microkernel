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

//! `challenge --host-keys F`: a fresh nonce, and the challenge the
//! machine's document must answer for these host keys. The machine is asked
//! for a document over the challenge; `body` is given the nonce.

use nsh_enrol::keys::PUBLIC_FILE_LEN;
use nsh_enrol::{hex, Error};

use super::args::Args;
use nsh_enrol::files::read_exact;

pub fn run(a: &Args) -> Result<(), Error> {
    let keys = read_exact::<PUBLIC_FILE_LEN>(a.one("host-keys")?)?;
    let mut nonce = [0u8; 32];
    if !nsh_enrol::rand::fill(&mut nonce) {
        return Err(Error::Randomness);
    }
    println!("nonce      {}", hex::of(&nonce));
    println!("challenge  {}", hex::of(&nsh_enrol::challenge(&nonce, &keys)));
    Ok(())
}
