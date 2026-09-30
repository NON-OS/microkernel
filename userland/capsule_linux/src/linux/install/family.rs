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

use crate::linux::file::family::{chosen, Family};

pub fn install(pkg: &str, pin: &[u8; 32]) -> Result<(), super::Why> {
    /*
     * A tier the personality ships is installed from the image, not a mirror.
     */
    if let Some(app) = super::apps::app(pkg) {
        return super::apps_install::install(app, pin);
    }
    match chosen() {
        Family::Pacman => super::pacman::install(pkg, pin),
        Family::Debian => super::deb::install(pkg, pin),
        Family::Alpine => super::run::install(pkg, pin),
    }
}
