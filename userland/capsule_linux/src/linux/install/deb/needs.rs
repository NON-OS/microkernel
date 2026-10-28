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

//! A NONOS-mirror program's Debian runtime: the packages its content pins
//! (pinned.rs), each fetched from the Debian mirror by its pool file, held
//! to its pinned SHA-256 and placed as a package of its own, before the
//! program itself is placed. No index is read: the pins are already the
//! market's. Content with no pins installs as before. Pins in a family
//! other than Debian are refused, since a glibc program has no loader there.

use super::fetch::fetch;
use super::pinned::{needs_path, pinned};
use super::source::source;
use crate::linux::file::family::{chosen, Family};
use crate::linux::install::auth::Verified;
use crate::linux::install::place::all::unpack;
use crate::linux::install::refuse::refuse as say;
use crate::linux::install::tar::walk;
use crate::linux::install::tar_kind::Kind;
use crate::linux::install::Why;

pub fn install_needs(content: &Verified, tool: &str) -> Result<(), Why> {
    let at = needs_path(tool);
    let walked = walk(content.files());
    let Some(text) = walked.entries.iter().rev().find(|e| e.name == at.as_bytes()) else {
        return Ok(());
    };
    if !matches!(text.kind, Kind::File) {
        return say(b"[LINUX] refused: its Debian pins are not a file\n", Why::PACKAGE);
    }
    if chosen() != Family::Debian {
        return say(
            b"[LINUX] refused: it pins Debian packages outside the Debian family\n",
            Why::PACKAGE,
        );
    }
    let Some(needs) = core::str::from_utf8(&text.body).ok().and_then(pinned) else {
        return say(b"[LINUX] refused: its Debian pins do not parse\n", Why::PACKAGE);
    };
    drop(walked);
    let Some(src) = source() else {
        return say(b"[LINUX] this image was built without a Debian mirror\n", Why::NO_MIRROR);
    };
    super::super::http::reachable(src.at())?;
    super::merged_usr::lay_out()?;
    for rec in &needs {
        let Some(files) = fetch(&src, rec, None) else {
            return Err(Why::PACKAGE);
        };
        unpack(&files, &rec.name, false, true)?;
    }
    Ok(())
}
