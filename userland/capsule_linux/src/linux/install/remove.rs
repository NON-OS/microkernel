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

//! `uninstall <name>`: take out what a package's install put in.
//!
//! Its manifest (`package_record`) says which files it wrote and which link
//! lines it added. Each file goes, with any trailer minted beside it, unless
//! another installed package lists it too; its link lines go from the table;
//! its program record goes, so `run` no longer finds it. What would not go
//! stays in the manifest, so asking again finishes the job rather than
//! failing on what is already gone. The packages it pulled in stay: each is
//! a package of its own, and another install may need it.
//!
//! A shipped Qwen tier keeps its program, which is part of the image; its
//! model files go off the data volume through the model fetcher, which holds
//! the right the kernel asks for to remove them (`model_remove`).

use alloc::format;

use nonos_libc::mk_debug;

use super::apps::{wanted, Wanted};
use super::manifest::{only_mine, Manifest};
use super::model_remove::{remove_not_started, removed};
use super::place_links::unrecord;
use super::Why;
use crate::linux::attest_paths::beside;
use crate::linux::file::{key, store_stat, store_unlink};

pub fn uninstall(pkg: &str) -> Result<(), Why> {
    match wanted(pkg) {
        Wanted::Tier(app) => return tier(pkg, app.tier()),
        Wanted::UnknownTier => return Err(Why::ModelUnknown),
        Wanted::Package => {}
    }
    let Some(mine) = super::package_record::read(pkg) else {
        say(&format!("[LINUX] {pkg}: no install of it is recorded on this system\n"));
        return Err(Why::NotInstalled);
    };
    let others = super::package_record::others(pkg);
    let files = only_mine(&mine.files, &others, |m| &m.files);
    let links = only_mine(&mine.links, &others, |m| &m.links);
    let mut left = Manifest::default();
    for at in &files {
        // Gone already is as good as taken out.
        if store_unlink(&key(at)).is_err() && store_stat(&key(at)).is_ok() {
            left.files.push(at.clone());
        }
        let _ = store_unlink(&key(&beside(at, b".zk_trailer.bin")));
    }
    if !links.is_empty() && !unrecord(&links) {
        left.links = links.clone();
    }
    let forgot = super::program::forget(pkg);
    let settled = super::package_record::settle(pkg, &left);
    say(&format!(
        "[LINUX] {pkg}: took out {} of {} file(s) and {} link(s); {} shared with another package stay\n",
        files.len() - left.files.len(),
        files.len(),
        links.len() - left.links.len(),
        mine.files.len() - files.len(),
    ));
    match left.files.is_empty() && left.links.is_empty() && forgot && settled {
        true => Ok(()),
        false => Err(Why::RemovePartial),
    }
}

/// A shipped tier: its program is the image's and stays; its model files go
/// off the data volume by the model fetcher's `remove` (`model_remove`).
fn tier(pkg: &str, tier: &str) -> Result<(), Why> {
    let status = super::fetcher::remove(tier).map_err(|rc| {
        say(&format!("[LINUX] {pkg}: the model fetcher did not start, errno {}\n", -rc));
        remove_not_started(rc)
    })?;
    say(&format!("[LINUX] {pkg}: the model fetcher removed its model, status {status}\n"));
    removed(status)
}

fn say(line: &str) {
    let _ = mk_debug(line.as_ptr(), line.len());
}
