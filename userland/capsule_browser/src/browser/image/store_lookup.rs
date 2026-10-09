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

use alloc::collections::BTreeMap;
use alloc::string::String;

use super::store::{Decoded, Store};
use crate::browser::url::{join, Scheme, Url};

/* Joins remembered before the memo starts over. */
const MAX_JOINED: usize = 4096;

/// Page-relative sources already resolved, for the base they were
/// resolved against.
#[derive(Default)]
pub(super) struct Joined {
    base: Option<(bool, String, u16, String)>,
    abs: BTreeMap<String, String>,
}

impl Store {
    /// The raster a page draws for `src`, resolved against `base` as the
    /// fetch resolved it. Each source is joined once per base; painting
    /// every frame then costs one lookup, not a URL join.
    pub fn ready_for(&self, base: Option<&Url>, src: &str) -> Option<&Decoded> {
        match base {
            Some(b) => self.with_abs(b, src, |abs| self.ready(abs)),
            None => self.ready(src),
        }
    }

    /// The natural size of the image a page draws for `src`.
    pub fn natural_for(&self, base: Option<&Url>, src: &str) -> Option<(u32, u32)> {
        match base {
            Some(b) => self.with_abs(b, src, |abs| self.natural(abs)),
            None => self.natural(src),
        }
    }

    pub(super) fn with_abs<R>(&self, b: &Url, src: &str, f: impl FnOnce(&str) -> R) -> R {
        let tls = b.scheme == Scheme::Https;
        let mut memo = self.joined.borrow_mut();
        let same = memo.base.as_ref().is_some_and(|(s, h, p, path)| {
            *s == tls && *h == b.host && *p == b.port && *path == b.path
        });
        if !same {
            memo.base = Some((tls, b.host.clone(), b.port, b.path.clone()));
            memo.abs = BTreeMap::new();
        }
        if let Some(abs) = memo.abs.get(src) {
            return f(abs);
        }
        if memo.abs.len() >= MAX_JOINED {
            memo.abs.clear();
        }
        let abs = join(b, src);
        let found = f(&abs);
        memo.abs.insert(String::from(src), abs);
        found
    }
}
