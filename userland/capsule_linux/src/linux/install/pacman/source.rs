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

//! Where pacman packages come from, fixed when the image is built. An image
//! built without a mirror has none: no address or repository is guessed.
//!
//! NONOS_PACMAN_MIRROR is a.b.c.d:port, NONOS_PACMAN_HOST the Host line,
//! NONOS_PACMAN_PATH a path with `{repo}` in it, NONOS_PACMAN_REPOS the
//! repositories to search, comma-separated, in order.

use alloc::format;
use alloc::vec::Vec;

use super::super::http::get_as;

pub struct Source {
    ip: &'static str,
    port: u16,
    host: &'static str,
    path: &'static str,
}

pub fn source() -> Option<Source> {
    let at = option_env!("NONOS_PACMAN_MIRROR")?;
    let (ip, port) = at.rsplit_once(':').unwrap_or((at, "80"));
    let host = option_env!("NONOS_PACMAN_HOST")?;
    let path = option_env!("NONOS_PACMAN_PATH")?;
    Some(Source { ip, port: port.parse().ok()?, host, path })
}

pub fn repos() -> impl Iterator<Item = &'static str> {
    option_env!("NONOS_PACMAN_REPOS").unwrap_or("").split(',').filter(|r| !r.is_empty())
}

impl Source {
    pub fn get(&self, repo: &str, file: &str) -> Option<Vec<u8>> {
        let dir = self.path.replace("{repo}", repo);
        get_as(self.ip, self.port, self.host, &format!("{dir}/{file}"))
    }
}
