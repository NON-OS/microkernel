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

//! `install <name>`: fetch a package and everything under it.

use alloc::string::String;
use alloc::vec::Vec;

use nonos_libc::mk_debug;

use super::fetch::get::fetch;
use super::get::closure::{closure, Closed};
use super::get::index::load_index;
use super::get::package::path_of;
use super::get::window::window;
use super::held_keep::keeps;
use super::place::all::unpack;
use super::timing;

pub(super) const RELEASE: &str = "v3.20";
pub(super) const ARCH: &str = "x86_64";
pub(super) const BRANCHES: [&str; 2] = ["main", "community"];

/// Alpine's install, extras and all; `family::install` sends the other families elsewhere.
pub fn install(name: &str, pin: &[u8; 32], extras: &[&str]) -> Result<(), super::Why> {
    timing::timed(|| install_all(name, pin, extras))
}

fn install_all(name: &str, pin: &[u8; 32], extras: &[&str]) -> Result<(), super::Why> {
    super::http::reach::reachable(super::mirror::mirror().0)?;
    let Some(index) = load_index() else {
        say(b"[LINUX] no package index\n");
        return Err(super::Why::INDEX);
    };
    timing::packages();
    let (max, have) = (super::limit::max_packages(), super::package_record::installed());
    let pkgs = closure(&index, name, extras, &have, max).map_err(|why| match why {
        Closed::NotProvided => refused("nothing provides it", super::Why::NOT_PROVIDED),
        Closed::TooLarge => refused(
            &alloc::format!("refused: closure passes {max} packages"),
            super::Why::TOO_LARGE,
        ),
    })?;
    // Fetched in a window (get/window.rs), verified and placed in this order.
    let paths: Option<Vec<String>> = pkgs.iter().map(|p| path_of(p)).collect();
    let paths = paths.ok_or(super::Why::INDEX)?;
    let sizes: Vec<usize> = pkgs.iter().map(|p| p.size).collect();
    window(&paths, &sizes, &mut |at, apk| {
        let pkg = pkgs[at];
        let files = fetch(pkg, apk.unwrap_or_default(), (pkg.name == name).then_some(pin))?;
        say(b"[LINUX] provenance Verified: index signature and checksums match\n");
        match timing::placing(|| unpack(&files, &pkg.name, pkg.name == name, false)) {
            Err(why) if keeps(why == super::Why::IN_USE, &pkg.name, name, &have) => {
                say(alloc::format!("[LINUX] {}: a running app holds it; its copy in the store is kept\n", pkg.name).as_bytes());
                Ok(())
            }
            done => done.map(drop),
        }
    })?;
    let names: Vec<String> = pkgs.iter().map(|p| p.name.clone()).collect();
    super::record::closure::keep(name, &names)
}

fn refused(why: &str, code: super::Why) -> super::Why {
    say(alloc::format!("[LINUX] {why}\n").as_bytes());
    code
}

fn say(line: &[u8]) {
    let _ = mk_debug(line.as_ptr(), line.len());
}
