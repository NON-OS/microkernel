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

//! `escrow-serve` (B5b, plan/29): the seller's agent for one release. It
//! answers each buyer's request with a fresh K sealed to that buyer, keeps K
//! in --keys before the answer leaves, and prunes that file at start. It
//! never signs or sends a transaction; deliver() is the operator's step with
//! the bond's agent key, reading the file.

use std::net::TcpListener;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use rand::rngs::OsRng;
use rand::RngCore;

use crate::args::Flags;
use crate::error::CliError;
use crate::escrow::{prune, read_request, write_answer, Agent, Bucket, Seen, Served};

fn key32(flags: &Flags, name: &'static str) -> Result<[u8; 32], CliError> {
    let v = hex::decode(flags.require(name).map_err(CliError::Json)?.trim_start_matches("0x"))
        .map_err(|e| CliError::Json(format!("--{name}: {e}")))?;
    let got = v.len();
    v.try_into().map_err(|_| CliError::BadHexLen { field: name, expected: 32, got })
}

fn now_ms() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_millis() as u64)
}

pub fn run(flags: &Flags) -> Result<(), CliError> {
    let served = Served {
        listing_key: key32(flags, "listing-key")?,
        release_key: key32(flags, "release-key")?,
        content: std::fs::read(flags.require("package").map_err(CliError::Json)?)?,
    };
    let per_minute = match flags.optional("per-minute") {
        Some(v) => v.parse::<u32>().map_err(|e| CliError::Json(format!("--per-minute: {e}")))?,
        None => 30,
    };
    let keys = Path::new(flags.require("keys").map_err(CliError::Json)?);
    eprintln!("escrow-serve: pruned {} key lines past their life", prune(keys, now_ms() / 1000)?);
    let listener = TcpListener::bind(flags.optional("listen").unwrap_or("127.0.0.1:8640"))?;
    let bucket = Bucket::new(per_minute, now_ms());
    let mut agent = Agent { served: &served, seen: Seen::default(), bucket, keys };
    let mut draw = || {
        let mut b = [0u8; 32];
        OsRng.fill_bytes(&mut b);
        b
    };
    for stream in listener.incoming() {
        let Ok(mut s) = stream else { continue };
        let (status, out) = match read_request(&mut s) {
            Some(body) => agent.serve(&body, now_ms(), &mut draw)?,
            None => (400, Vec::new()),
        };
        let _ = write_answer(&mut s, status, &out);
    }
    Ok(())
}
