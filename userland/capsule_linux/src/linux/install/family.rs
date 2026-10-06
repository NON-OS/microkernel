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

//! Which distribution an install is for. A family is named, never inferred
//! from the package: `pacman:` and `deb:` say so, and a bare name is Alpine's.
//! The name was split when the process started (`file::family::choose`).

use nonos_libc::mk_debug;

use super::apps::{wanted, Wanted};
use crate::linux::file::family::{chosen, Family};

pub fn install(pkg: &str, pin: &[u8; 32]) -> Result<(), super::Why> {
    /*
     * A tier the personality ships is installed from the image, its model
     * brought as a dependency. A name in the tiers' namespace that is no
     * shipped tier is refused by name: a model is never looked up as a
     * package, where the index could only answer that nothing provides it.
     */
    match wanted(pkg) {
        Wanted::Tier(app) => return super::apps_install::install(app, pin),
        Wanted::UnknownTier => {
            let line = alloc::format!("[LINUX] {pkg}: no shipped Qwen tier has this name\n");
            let _ = mk_debug(line.as_ptr(), line.len());
            return Err(super::Why::ModelUnknown);
        }
        Wanted::Package => {}
    }
    /*
     * An in-tree tool the image does not carry: its content from the NONOS
     * package mirror, pinned by the market, into Alpine's tree, where the
     * stored tools are.
     */
    if let Some(tool) = super::tools::tool(pkg) {
        return super::tools_install::install(tool, pin);
    }
    match chosen() {
        Family::Pacman => super::pacman::install(pkg, pin),
        Family::Debian => super::deb::install(pkg, pin),
        Family::Alpine => super::run::install(pkg, pin),
    }
}
