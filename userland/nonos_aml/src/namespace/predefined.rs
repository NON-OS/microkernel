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

//! The objects that exist before any table is loaded.

use super::{Entry, Kind, MethodInfo, Namespace, Origin, Predefined, Seg, ROOT};
use alloc::collections::BTreeMap;
use alloc::vec::Vec;

/// Section 5.3.1 names the root scopes, and section 5.7 the predefined
/// objects the operating system supplies: the Global Lock mutex, the \_OS
/// name, the \_OSI method (one argument) and the \_REV revision. What \_OS,
/// \_OSI and \_REV answer is the interpreter's business; here they only
/// exist, so a table cannot define its own in their place.
const ROOT_OBJECTS: [(Seg, Kind, Predefined); 9] = [
    (*b"_GPE", Kind::Scope, Predefined::Gpe),
    (*b"_PR_", Kind::Scope, Predefined::Pr),
    (*b"_SB_", Kind::Device, Predefined::Sb),
    (*b"_SI_", Kind::Scope, Predefined::Si),
    (*b"_TZ_", Kind::Scope, Predefined::Tz),
    (*b"_GL_", Kind::Mutex, Predefined::Gl),
    (*b"_OS_", Kind::Name, Predefined::Os),
    (*b"_OSI", Kind::Method(MethodInfo { flags: 1 }), Predefined::Osi),
    (*b"_REV", Kind::Name, Predefined::Rev),
];

impl Namespace {
    /// An empty namespace holding only the root, before the predefined
    /// objects are installed.
    pub(super) fn bare() -> Self {
        let root = Entry {
            seg: *b"\\___",
            parent: ROOT,
            kind: Kind::Scope,
            origin: Origin::Predefined(Predefined::Root),
            gen: 0,
            live: true,
            first: None,
            last: None,
            next: None,
        };
        let (free, journal) = (Vec::new(), Vec::new());
        Self { entries: alloc::vec![root], index: BTreeMap::new(), free, journal, journaling: 0 }
    }

    pub fn new() -> Self {
        let mut ns = Self::bare();
        for (seg, kind, which) in ROOT_OBJECTS {
            // A fresh namespace has nothing under the root to collide with.
            let _ = ns.add(ROOT, seg, kind, Origin::Predefined(which));
        }
        ns
    }
}

impl Default for Namespace {
    fn default() -> Self {
        Self::new()
    }
}
