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

//! The verdict printed, and written to `--out` (signed beside it) when asked.
//! One not written or signed as asked is not delivered (exit 3), even a GO,
//! and no unsigned file is left in its place.

use crate::cli::CheckArgs;
use crate::ctx::Kind;
use crate::exit;
use crate::sign::sign;
use crate::verdict::{json, text, Overall, Verdict};
use crate::write::{sig_path, write_whole};
use ed25519_dalek::SigningKey;
use std::fs;

pub fn deliver(v: &Verdict, args: &CheckArgs, key: Option<&SigningKey>) -> u8 {
    let body = match args.json {
        false => text::render(v),
        true => match json::render(v) {
            Ok(b) => b,
            Err(e) => return refuse(&e),
        },
    };
    print!("{body}");
    if let Some(out) = &args.out {
        if let Err(e) = write_whole(out, body.as_bytes()) {
            return refuse(&format!("the verdict was not written to {}: {e}", out.display()));
        }
        if let Some(k) = key {
            let sig = sig_path(out);
            if let Err(e) = write_whole(&sig, sign(k, body.as_bytes()).as_bytes()) {
                let _ = fs::remove_file(out);
                return refuse(&format!("the signature was not written to {}: {e}", sig.display()));
            }
        }
    }
    match (v.overall, v.kind) {
        (Overall::Go, Kind::Development) => exit::DEV_GO,
        (Overall::Go, _) => exit::GO,
        (Overall::NoGo, _) => exit::NO_GO,
    }
}

fn refuse(why: &str) -> u8 {
    eprintln!("nonos-harden: {}", text::clean(why));
    exit::NOT_DELIVERED
}
