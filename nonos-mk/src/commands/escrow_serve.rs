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

//! `escrow-serve` (B5b, the plan step 1): the seller's agent for one release.
//! It answers each buyer's request with a fresh K, sealed to that buyer, and
//! keeps K in --keys before the answer leaves. It never signs or sends a
//! transaction; deliver() is the operator's step with the bond's agent key.

use std::net::TcpListener;
use std::path::Path;

use rand::rngs::OsRng;
use rand::RngCore;

use crate::args::Flags;
use crate::error::CliError;
use crate::escrow::{answer, read_request, record, write_answer, Refusal, Seen, Served};

fn key32(flags: &Flags, name: &'static str) -> Result<[u8; 32], CliError> {
    let v = hex::decode(flags.require(name).map_err(CliError::Json)?.trim_start_matches("0x"))
        .map_err(|e| CliError::Json(format!("--{name}: {e}")))?;
    let got = v.len();
    v.try_into().map_err(|_| CliError::BadHexLen { field: name, expected: 32, got })
}

pub fn run(flags: &Flags) -> Result<(), CliError> {
    let served = Served {
        listing_key: key32(flags, "listing-key")?,
        release_key: key32(flags, "release-key")?,
        content: std::fs::read(flags.require("package").map_err(CliError::Json)?)?,
    };
    let keys = Path::new(flags.require("keys").map_err(CliError::Json)?);
    let listener = TcpListener::bind(flags.optional("listen").unwrap_or("127.0.0.1:8640"))?;
    let mut seen = Seen::default();
    let mut draw = || {
        let mut b = [0u8; 32];
        OsRng.fill_bytes(&mut b);
        b
    };
    for stream in listener.incoming() {
        let Ok(mut s) = stream else { continue };
        let Some(body) = read_request(&mut s) else {
            let _ = write_answer(&mut s, 400, &[]);
            continue;
        };
        let (status, out) = match answer(&served, &mut seen, &body, &mut draw) {
            Ok((sealed, k)) => match record(keys, &k) {
                Ok(()) => (200, sealed),
                Err(e) => return Err(CliError::Io(e)),
            },
            Err(Refusal::Seen) => (429, Vec::new()),
            Err(Refusal::NotServed) => (404, Vec::new()),
            Err(_) => (400, Vec::new()),
        };
        let _ = write_answer(&mut s, status, &out);
    }
    Ok(())
}
