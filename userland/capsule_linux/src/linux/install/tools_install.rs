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


//! `install nonos-<tool>`: an in-tree tool the image does not carry, from
//! the NONOS package mirror this image was built for (NONOS_PACKAGE_MIRROR,
//! name:port, from nonos.toml's linux_packages). The content is fetched
//! through the network the person chose, held to the market's pin, and
//! placed like any package: written into the tree, vouched for, recorded so
//! it can be run and taken out again. Proof files are not part of it, so the
//! trust it is run under is the same as any installed package's.

use nonos_libc::mk_debug;

use super::auth::Verified;
use super::http::{get_as, reachable};
use super::mirror::parse;
use super::place::unpack;
use super::tools::{content_path, PREFIX};
use super::Why;

pub fn install(tool: &str, pin: &[u8; 32]) -> Result<(), Why> {
    let Some(at) = option_env!("NONOS_PACKAGE_MIRROR").filter(|m| !m.is_empty()) else {
        say(b"[LINUX] this image was built without a NONOS package mirror\n");
        return Err(Why::NoMirror);
    };
    let (name, port, _) = parse(at);
    reachable(name)?;
    let Some(content) = get_as(name, port, name, &content_path(pin)) else {
        say(b"[LINUX] the tool did not download\n");
        return Err(Why::Package);
    };
    if blake3::hash(&content).as_bytes() != pin {
        say(b"[LINUX] the tool is not the one the market listed\n");
        return Err(Why::Package);
    }
    say(b"[LINUX] provenance Verified: the market's pin matches\n");
    // Held to the signed index's pin above, so these are authenticated bytes.
    let files = Verified::checked(content);
    let listing = alloc::format!("{PREFIX}{tool}");
    unpack(&files, &listing, true).map(|_| ())
}

fn say(line: &[u8]) {
    let _ = mk_debug(line.as_ptr(), line.len());
}
