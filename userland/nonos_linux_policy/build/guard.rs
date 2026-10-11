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

//! Whether an empty vendor table may be built. A shipping build with no
//! table would run every vendor program in the owner's default box, without
//! a word, so it fails instead and says why. A plain dev build, and every host
//! build of the tests, keeps the empty table.

pub struct Build<'a> {
    pub target: &'a str,
    pub host: &'a str,
    pub profile: &'a str,
    pub in_flake: bool,
}

impl Build<'_> {
    /// Every shipping artifact is built for a NONOS target, in release or by
    /// the flake's lanes; a host build is a test.
    pub fn ships(&self) -> bool {
        self.target != self.host && (self.profile == "release" || self.in_flake)
    }
}

/// `userland` is whether NONOS_LINUX_USERLAND was set; `rows`, how many
/// vendor programs the table holds.
pub fn may_build(build: &Build<'_>, userland: bool, rows: usize) -> Result<(), String> {
    if !build.ships() {
        return Ok(());
    }
    if !userland {
        return Err("NONOS_LINUX_USERLAND is not set, so the vendor table would be empty \
                    and every vendor program would run in the default box"
            .into());
    }
    if rows == 0 {
        return Err("policies.txt names no program, so the vendor table is empty".into());
    }
    Ok(())
}
