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

//! A TermList read inside the scope its object names.

use super::parser::Parser;
use super::Scopes;
use crate::error::Error;
use crate::grammar::{Cursor, NamePath};

impl<S: Scopes> Parser<'_, S> {
    /// The namespace is told before and after, so the names called inside
    /// resolve from the object's scope; it hears `leave` on failure too.
    pub(super) fn scoped(
        &mut self,
        c: &mut Cursor<'_>,
        name: Option<NamePath>,
    ) -> Result<u32, Error> {
        let Some(name) = name else { return self.terms(c) };
        self.scopes.enter(self.bytes, name);
        let r = self.terms(c);
        self.scopes.leave();
        r
    }
}
