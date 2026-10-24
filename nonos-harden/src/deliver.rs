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

//! Printing the verdict, and writing it to `--out` when asked. A verdict that
//! cannot be written where it was asked for is not delivered (exit 3), even a
//! GO: an operator who asked for the file must not go on without it. The file
//! is written whole or not at all.

use crate::cli::CheckArgs;
use crate::exit;
use crate::verdict::{json, text, Overall, Verdict};
use std::fs;
use std::io;
use std::path::Path;

pub fn deliver(v: &Verdict, args: &CheckArgs) -> u8 {
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
    }
    match v.overall {
        Overall::Go => exit::GO,
        Overall::NoGo => exit::NO_GO,
    }
}

fn refuse(why: &str) -> u8 {
    eprintln!("nonos-harden: {}", text::clean(why));
    exit::NOT_DELIVERED
}

/// Write beside the target, then rename over it, so a reader never sees half.
pub fn write_whole(path: &Path, bytes: &[u8]) -> io::Result<()> {
    let mut partial = path.as_os_str().to_owned();
    partial.push(".partial");
    fs::write(&partial, bytes)?;
    fs::rename(&partial, path).inspect_err(|_| {
        let _ = fs::remove_file(&partial);
    })
}
