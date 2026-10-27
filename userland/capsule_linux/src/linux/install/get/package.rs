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

//! Where one package is fetched: the branch whose signed index listed it.

use alloc::format;
use alloc::string::String;

use super::super::index::Pkg;
use super::super::run::{ARCH, BRANCHES, RELEASE};

/// Where `pkg` is fetched: from its own branch only, as asking another first
/// cost a 404 and a stream for every community package.
pub(in crate::linux::install) fn path_of(pkg: &Pkg) -> Option<String> {
    let branch = BRANCHES.get(pkg.branch)?;
    let (name, version) = (&pkg.name, &pkg.version);
    Some(format!("/alpine/{RELEASE}/{branch}/{ARCH}/{name}-{version}.apk"))
}
