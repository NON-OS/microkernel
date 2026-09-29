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

//! Where Debian packages come from, fixed when the image is built. An image
//! built without a mirror has none: no address or suite is guessed.
//!
//! NONOS_DEB_MIRROR is a.b.c.d:port, NONOS_DEB_HOST the Host line,
//! NONOS_DEB_ROOT the archive's path (`/kali`), NONOS_DEB_SUITE the suite
//! (`kali-rolling`), NONOS_DEB_COMPONENTS the components, comma-separated.

use alloc::format;
use alloc::vec::Vec;

use super::super::http::get_as;

pub struct Source {
    ip: &'static str,
    port: u16,
    host: &'static str,
    root: &'static str,
    pub suite: &'static str,
}

pub fn source() -> Option<Source> {
    let at = option_env!("NONOS_DEB_MIRROR")?;
    let (ip, port) = at.rsplit_once(':').unwrap_or((at, "80"));
    let host = option_env!("NONOS_DEB_HOST")?;
    let (root, suite) = (option_env!("NONOS_DEB_ROOT")?, option_env!("NONOS_DEB_SUITE")?);
    Some(Source { ip, port: port.parse().ok()?, host, root, suite })
}

pub fn components() -> impl Iterator<Item = &'static str> {
    option_env!("NONOS_DEB_COMPONENTS").unwrap_or("main").split(',').filter(|c| !c.is_empty())
}

impl Source {
    /// A path under the archive root: `dists/...` or a pool file name.
    pub fn get(&self, path: &str) -> Option<Vec<u8>> {
        get_as(self.ip, self.port, self.host, &format!("{}/{path}", self.root))
    }
}
