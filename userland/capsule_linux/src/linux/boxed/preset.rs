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

//! The box a `linux --box` run is held in: `strict`, no network and one
//! scratch folder, the run's own /tmp; `offline`, no network and the image
//! read-only, the run's own folders writable; `allow:<item>,...`, only the
//! folders (`/tmp/out`) and destinations (`net:<address or name>:<port>`)
//! named. Each grants a terminal and execve, so a script runs its tools,
//! which are never let do more than the box (identity_exec.rs).

use alloc::vec::Vec;

use nonos_linux_policy::mask::{EXEC, FILES_HOME, NET_ANON, TTY};

use super::items::allowed;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Preset {
    Strict,
    Offline,
    Allow,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BoxPolicy {
    pub preset: Preset,
    /// The services the box leaves (nonos_linux_policy::mask).
    pub mask: u64,
    /// The folders writable, each canonical, sorted and once.
    pub folders: Vec<Vec<u8>>,
    /// The destinations reachable, as a canonical list (dests.rs); empty for none.
    pub dests: Vec<u8>,
}

/// The run's own folders, as offline leaves them writable.
pub const OWN: &[&[u8]] = &[b"/dev/shm", b"/home", b"/root", b"/run", b"/tmp", b"/var/tmp"];

pub fn parse(word: &[u8]) -> Result<BoxPolicy, &'static str> {
    let (preset, mask, folders, dests) = match word {
        b"strict" => (Preset::Strict, TTY | EXEC, alloc::vec![b"/tmp".to_vec()], Vec::new()),
        b"offline" => (Preset::Offline, TTY | EXEC | FILES_HOME, own(), Vec::new()),
        _ => {
            let items = word.strip_prefix(b"allow:").ok_or("not strict, offline or allow:")?;
            let (folders, dests) = allowed(items)?;
            let net = if dests.is_empty() { 0 } else { NET_ANON };
            (Preset::Allow, TTY | EXEC | FILES_HOME | net, folders, dests)
        }
    };
    Ok(BoxPolicy { preset, mask, folders, dests })
}

fn own() -> Vec<Vec<u8>> {
    OWN.iter().map(|f| f.to_vec()).collect()
}
