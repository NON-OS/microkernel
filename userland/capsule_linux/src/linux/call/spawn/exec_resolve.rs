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

//! Which image actually runs, once `#!` has had its say.

use crate::linux::guest::Links;
use alloc::vec::Vec;

use crate::linux::abi::errno;
use crate::linux::app::{bind, AppIdentity};
use crate::linux::file::{key, store_read, visible};

use super::exec_built_in::built_in;
use super::exec_rewrite::rewrite;
use super::exec_shebang::parse;

/// The same ceiling the first image is read under.
pub const MAX_IMAGE: u32 = 64 << 20;

/// Linux's own limit on how deep `#!` may point at another script.
const MAX_DEPTH: usize = 4;

pub struct Program {
    pub path: Vec<u8>,
    pub bytes: Vec<u8>,
    pub argv: Vec<Vec<u8>>,
    /// The file execve named, bound to its policy: a script's, not its
    /// interpreter's, since the script is what was published.
    pub app: AppIdentity,
}

/// A path reached through a link is loaded, and proved, as the file it names.
pub fn resolve(links: &Links, cwd: &[u8], name: &[u8], argv: &[Vec<u8>]) -> Result<Program, u64> {
    let mut path = links.follow(visible(cwd, name), true);
    let mut args = argv.to_vec();
    let mut named: Option<AppIdentity> = None;
    for _ in 0..MAX_DEPTH {
        let Ok(bytes) = store_read(&key(&path), MAX_IMAGE) else {
            return built_in(&path, args, named);
        };
        if crate::linux::attest::verify(&path, &bytes).is_err() {
            return Err(errno::fail(errno::EPERM));
        }
        let app = match named {
            Some(app) => app,
            None => bind(&path, &bytes).map_err(|_| errno::fail(errno::EPERM))?,
        };
        named = Some(app);
        let Some(interp) = parse(&bytes) else {
            return Ok(Program { path, bytes, argv: args, app });
        };
        args = rewrite(&interp, &path, &args);
        path = links.follow(visible(cwd, &interp.path), true);
    }
    Err(errno::fail(errno::ELOOP))
}
