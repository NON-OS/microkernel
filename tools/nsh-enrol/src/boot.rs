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

//! The machine's boot applications: every application its firmware
//! measured into PCR 4 that is not the release loader, each once, in the
//! order they ran. Exactly one release loader must have run, and every other
//! application must be one the operator expects: nothing the firmware
//! started is pinned unseen.

use nsh_wire::MAX_BOOT_APPS;

use crate::Error;

pub fn applications(
    ran: &[[u8; 32]],
    loaders: &[[u8; 32]],
    expected: &[[u8; 32]],
) -> Result<Vec<[u8; 32]>, Error> {
    let mut seen: Vec<&[u8; 32]> = ran.iter().filter(|a| loaders.contains(a)).collect();
    seen.sort();
    seen.dedup();
    match seen.len() {
        0 => return Err(Error::NoLoader),
        1 => {}
        _ => return Err(Error::Value("loaders: more than one release loader ran")),
    }
    let mut apps: Vec<[u8; 32]> = Vec::new();
    for a in ran.iter().filter(|a| !loaders.contains(a)) {
        if a.iter().all(|b| *b == 0) {
            return Err(Error::Value("boot application digest"));
        }
        if !expected.contains(a) {
            return Err(Error::Unexpected(*a));
        }
        if !apps.contains(a) {
            apps.push(*a);
        }
    }
    match apps.len() {
        n if n > MAX_BOOT_APPS => Err(Error::BootApplications(n)),
        _ => Ok(apps),
    }
}
